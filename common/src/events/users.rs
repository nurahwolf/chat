/// A client has joined.
pub struct Users {
	pub user: crate::User,
}

impl Users {
	pub fn new(user: crate::User) -> Self {
		Self { user }
	}
}

impl crate::Event for Users {
	fn handle(&mut self, mgr: &mut crate::UserManager) -> crate::Response {
		crate::Response::new(mgr.to_string(), &self.user).skip_others()
	}
}
