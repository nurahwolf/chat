//! Possible the worlds worst chat client.

use std::net::{TcpStream, ToSocketAddrs};

use smol::{future, io, Async, Unblock};

const DNS_ADDR: &str = "barn.lunar.cloud:6969";
const IP_ADDR: ([u8; 4], u16) = ([81, 2, 94, 163], 6969);

async fn connect() -> io::Result<Async<TcpStream>> {
	let dns_addr = DNS_ADDR.to_socket_addrs().map(|mut dns| dns.next());

	// Attempt to resolve DNS, if successful return the TCP stream
	if let Ok(Some(dns_addr)) = dns_addr {
		tracing::info!("Attempting to connect to server {dns_addr}");
		return Async::<TcpStream>::connect(dns_addr).await;
	}

	// If an error was returned, raise it
	if let Err(why) = dns_addr {
		tracing::error!("Failed to resolve DNS: {why}");
	}

	// Fall back to hardcoded IP
	tracing::info!("Attempting to connect to IP {:?}:{}", IP_ADDR.0, IP_ADDR.1);
	Async::<TcpStream>::connect(IP_ADDR).await
}

fn main() -> io::Result<()> {
	tracing_subscriber::fmt::init();
	smol::block_on(async {
		// Connect to the server and create async stdin and stdout.
		let stream = connect().await?;
		let stdin = Unblock::new(std::io::stdin());
		let mut stdout = Unblock::new(std::io::stdout());

		// Initial Messages
		tracing::info!("Connected to {}", stream.get_ref().peer_addr()?);
		tracing::info!("My nickname: {}", stream.get_ref().local_addr()?);
		tracing::info!("Type a message and hit enter!\n");

		let reader = &stream;
		let mut writer = &stream;

		// Wait until the standard input is closed or the connection is closed.
		future::race(
			async {
				let res = io::copy(stdin, &mut writer).await;
				println!("Quit!");
				res
			},
			async {
				let res = io::copy(reader, &mut stdout).await;
				println!("Server disconnected!");
				res
			},
		)
		.await?;

		Ok(())
	})
}
