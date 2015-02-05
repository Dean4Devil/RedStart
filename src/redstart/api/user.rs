//! Authentication API User
//! Authentication is done via the abstract objects of 'users' and 'groups'

// Todo: Change this API to implement proper RBAC.

pub struct User
{
	username: &str,
}

impl User
{
	/// Create a new User object
	pub fn login(self, username: &str, password: &str) -> Result<User>
	{
		if username == "testuser" && password == "testpassword"
		{
			return Ok(User { username: username });
		}
		else
		{
			return Err();
		}
	}

	/// Log the user out and thus invalidate the User object
	pub fn logout(user: User) -> ()
	{

	}
}
