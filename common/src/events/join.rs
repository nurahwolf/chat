/// A client has joined.
pub struct Join {
	pub user: crate::User,
}

impl Join {
	pub fn new(user: crate::User) -> Self {
		Self { user }
	}
}

impl std::fmt::Display for Join {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{} has joined!]\n", self.user)
	}
}

impl crate::Event for Join {
	fn handle(&mut self, mgr: &mut crate::UserManager) -> crate::Response {
		mgr.update_or_insert_user(&self.user);

		crate::Response::new(self.to_string(), &self.user)
	}
}
