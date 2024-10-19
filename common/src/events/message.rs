use crate::User;

/// A client has sent a message.
pub struct Message {
	pub msg: String,
	pub user: User,
}

impl Message {
	pub fn new(msg: String, user: User) -> Self {
		Self { msg, user }
	}
}

impl std::fmt::Display for Message {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}]: {}\n", self.user, self.msg)
	}
}

impl crate::Event for Message {
	fn handle(&mut self, mgr: &mut crate::UserManager) -> crate::Response {
		mgr.update_userref_from_manager(&mut self.user);

		crate::Response::new(self.to_string(), &self.user).skip_self()
	}
}
