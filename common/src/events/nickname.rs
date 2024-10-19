use crate::User;

/// A client has changed its nickname.
pub struct Nickname {
	pub nickname: String,
	pub user: User,
}

impl Nickname {
	pub fn new(nickname: String, user: User) -> Self {
		Self { nickname, user }
	}
}

impl std::fmt::Display for Nickname {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{} is now known as '{}']\n", self.user.addr, self.nickname)
	}
}

impl crate::Event for Nickname {
	fn handle(&mut self, mgr: &mut crate::UserManager) -> crate::Response {
		mgr.update_user_name(&self.user, self.nickname.clone());

		crate::Response::new(self.to_string(), &self.user)
	}
}
