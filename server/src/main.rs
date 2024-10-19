//! A TCP chat server.
//!
//! First, start a server:
//! ```
//! cargo run --bin server
//! ```
//! Then, start clients:
//! ```
//! cargo run --bin client
//! ```

use std::net::{SocketAddr, TcpListener, TcpStream};

use async_channel::{bounded, Receiver, Sender};
use smol::{io, prelude::*, Async};

// Constants for server address
const SERVER_ADDR: ([u8; 4], u16) = ([0, 0, 0, 0], 6969);

fn main() -> io::Result<()> {
	smol::block_on(async {
		// Initialize the tracing subscriber for logging
		tracing_subscriber::fmt::init();

		// Create a listener for incoming client connections
		let server = Async::<TcpListener>::bind(SERVER_ADDR)?;
		let (tx, rx) = bounded(100);

		tracing::info!("Listening on {}", server.get_ref().local_addr()?);

		// Spawn a background thread for dispatching events to connected clients
		smol::spawn(client_handler(rx)).detach();

		// Loop to accept and handle incoming connections
		loop {
			let connection = server.accept().await?;
			server_handler(connection, tx.clone()).await?;
		}
	})
}

/// Handles incoming TCP requests and spawns tasks for each client.
async fn server_handler((stream, addr): (Async<TcpStream>, SocketAddr), tx: Sender<Box<dyn common::Event + Send>>) -> io::Result<()> {
	let user = common::User::new(addr, stream);

	// Spawn a task to handle server messages and client lifecycle
	smol::spawn(async move {
		// Notify that the client has joined
		send_event(&tx, common::Join::new(user.clone())).await;

		// Handle client messages
		message_handler(&tx, user.clone()).await.ok();

		// Notify that the client has left
		send_event(&tx, common::Leave::new(user)).await;
	})
	.detach();

	Ok(())
}

/// Handles messages received from the client.
async fn message_handler(tx: &Sender<Box<dyn common::Event + Send>>, user: common::User) -> io::Result<()> {
	let mut lines = io::BufReader::new(user.stream.clone()).lines();

	// Process each line from the client's input
	while let Some(line) = lines.next().await {
		let line = line?;

		match line.split_ascii_whitespace().next().unwrap_or_default() {
			"/nick" => {
				// Handle nickname change request
				let new_nick = line.strip_prefix("/nick").unwrap_or_default().trim().to_owned();
				send_event(tx, common::Nickname::new(new_nick, user.clone())).await;
			}
			"/users" => {
				// Handle a requeset for a list of users
				send_event(tx, common::Users::new(user.clone())).await;
			}
			"/whoami" => {
				// Handle a requeset for knowing details about yourself
				send_event(tx, common::WhoAmI::new(user.clone())).await;
			}
			_ => {
				// Handle regular messages
				send_event(tx, common::Message::new(line.to_owned(), user.clone())).await;
			}
		}
	}

	Ok(())
}

/// Helper function to send events to the event dispatcher.
async fn send_event<E: common::Event + Send + 'static>(tx: &Sender<Box<dyn common::Event + Send>>, event: E) {
	tx.send(Box::new(event)).await.ok();
}

/// Dispatches events to all connected clients.
async fn client_handler(rx: Receiver<Box<dyn common::Event + Send>>) -> io::Result<()> {
	// Active clients map
	let mut clients = common::UserManager::default();

	// Handle incoming events from the event channel
	while let Ok(mut event) = rx.recv().await {
		let res = event.handle(&mut clients);

		// Display the event message in the server process, trimming off any newlines
		tracing::info!("{}", res.msg.trim_end());

		// Broadcast the event message to all active clients
		for user in clients.fetch_users_mut() {
			if user.addr == res.user.addr && !res.skip_self {
				user.stream.write_all(res.msg.as_bytes()).await.ok();
			}

			if user.addr != res.user.addr && !res.skip_others {
				user.stream.write_all(res.msg.as_bytes()).await.ok();
			}
		}
	}

	Ok(())
}
