use crate::User;

#[derive(Default)]
pub struct UserManager {
	users: std::collections::HashMap<std::net::SocketAddr, User>,
}

impl std::fmt::Display for UserManager {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for user in self.users.values() {
			match user.name {
				Some(ref name) => writeln!(f, "[{} - Known as '{name}']", user.addr)?,
				None => writeln!(f, "[{} - No Nickname Set']", user.addr)?,
			}
		}
		Ok(())
	}
}

impl UserManager {
	/// Fetches the username as stored in the manager, else falls back to the passed user.
	pub fn get_username(&self, user: &crate::User) -> String {
		self.users.get(&user.addr).map_or_else(|| user.to_string(), |u| u.to_string())
	}

	/// Fetches a user
	pub fn get_user<'a>(&'a self, user: &'a crate::User) -> &'a User {
		self.users.get(&user.addr).unwrap_or(user)
	}

	/// Update the passed user from the user manager, if the user manager contains the user.
	pub fn update_userref_from_manager(&self, user: &mut User) {
		if let Some(mgr_user) = self.users.get(&user.addr) {
			*user = mgr_user.clone();
		}
	}

	pub fn update_or_insert_user(&mut self, user: &User) {
		self.users.entry(user.addr).or_insert(user.clone());
	}

	pub fn update_user_name(&mut self, user: &User, name: String) {
		self.users
			.entry(user.addr)
			.and_modify(|user| user.name = Some(name))
			.or_insert(user.clone());
	}

	pub fn remove_user(&mut self, user: &User) {
		self.users.remove(&user.addr);
	}

	/// Return a reference to all users
	pub fn fetch_users(&self) -> std::collections::hash_map::Values<std::net::SocketAddr, User> {
		self.users.values()
	}

	pub fn fetch_users_mut(&mut self) -> std::collections::hash_map::ValuesMut<std::net::SocketAddr, User> {
		self.users.values_mut()
	}
}
