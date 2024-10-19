/// A client has joined.
pub struct WhoAmI {
	pub user: crate::User,
}

impl WhoAmI {
	pub fn new(user: crate::User) -> Self {
		Self { user }
	}
}

impl std::fmt::Display for WhoAmI {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(ref name) = self.user.name {
			return write!(f, "[{} - Known as '{name}']\n", self.user.addr);
		}

		write!(f, "[{}]\n", self.user.addr)
	}
}

impl crate::Event for WhoAmI {
	fn handle(&mut self, mgr: &mut crate::UserManager) -> crate::Response {
		mgr.update_userref_from_manager(&mut self.user);

		crate::Response::new(self.to_string(), &self.user).skip_others()
	}
}
