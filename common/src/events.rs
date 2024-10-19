pub mod join;
pub mod leave;
pub mod message;
pub mod nickname;
pub mod users;
pub mod whoami;

pub trait Event {
	/// Handle the input and return a message (String) output
	fn handle(&mut self, mgr: &mut crate::UserManager) -> crate::Response;
}
