use crate::User;

/// A client has left.
pub struct Leave {
	pub user: User,
}

impl Leave {
	pub fn new(user: User) -> Self {
		Self { user }
	}
}

impl std::fmt::Display for Leave {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{} has left!]\n", self.user)
	}
}

impl crate::Event for Leave {
	fn handle(&mut self, mgr: &mut crate::UserManager) -> crate::Response {
		mgr.update_userref_from_manager(&mut self.user);
		mgr.remove_user(&self.user);

		crate::Response::new(self.to_string(), &self.user)
	}
}
