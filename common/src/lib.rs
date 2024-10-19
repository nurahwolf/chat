pub use events::join::Join;
pub use events::leave::Leave;
pub use events::message::Message;
pub use events::nickname::Nickname;
pub use events::users::Users;
pub use events::whoami::WhoAmI;
pub use events::Event;
pub use user_manager::UserManager;

mod events;
mod user_manager;

pub const DEFAULT_CLIENT_ADDR: ([u8; 4], u16) = ([127, 0, 0, 1], 6969);
pub const DEFAULT_SERVER_ADDR: ([u8; 4], u16) = ([0, 0, 0, 0], 6969);

pub struct Response<'a> {
	pub msg: String,
	pub skip_self: bool,
	pub skip_others: bool,
	pub user: &'a User,
}

impl<'a> Response<'a> {
	pub fn new(msg: String, user: &'a User) -> Self {
		Self {
			msg,
			skip_self: false,
			skip_others: false,
			user,
		}
	}

	pub fn skip_self(mut self) -> Self {
		self.skip_self = true;
		self
	}

	pub fn skip_others(mut self) -> Self {
		self.skip_others = true;
		self
	}
}

#[derive(Clone)]
pub struct User {
	pub addr: std::net::SocketAddr,
	pub name: Option<String>,
	pub stream: async_dup::Arc<smol::Async<std::net::TcpStream>>,
}

impl User {
	pub fn new(addr: std::net::SocketAddr, stream: smol::Async<std::net::TcpStream>) -> Self {
		Self {
			addr,
			name: None,
			stream: async_dup::Arc::new(stream),
		}
	}
}

impl std::fmt::Display for User {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(ref name) = self.name {
			return write!(f, "{name}");
		}

		self.addr.fmt(f)
	}
}
