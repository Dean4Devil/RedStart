use iron::prelude::*;
use iron::status::{self, Status};

use redstart::Store;
use redstart::{Session, SessionStore};

pub struct User
{
    sessionstore: Store,
}
impl User
{
	pub fn new(sessionstore: Store) -> User
	{
		User { sessionstore: sessionstore }
	}

	pub fn call(&self, model: &str, req: &mut Request) -> (Status, String)
	{
        // The Store is a Arc so no problem cloning it.
        let login = Login::new(self.sessionstore.clone());
        let logout = Logout::new(self.sessionstore.clone());
		match model
		{
			"login" =>
			{
				login.call(req)
			},
			"logout" =>
			{
				logout.call(req)
			},
			_ =>
			{
				(status::NotFound, "".to_string())
			},
		}
	}
}

struct Login
{
    sessionstore: Store,
}
impl Login
{
    pub fn new(sessionstore: Store) -> Login
    {
        Login { sessionstore: sessionstore }
    }
    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        (status::NotImplemented, "".to_string())
    }
}

struct Logout
{
    sessionstore: Store,
}
impl Logout
{
    pub fn new(sessionstore: Store) -> Logout
    {
        Logout { sessionstore: sessionstore }
    }
    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        if req.extensions.contains::<Session>()
        {
            let session_key: String = req.extensions.get_mut::<Session>().unwrap().clone();
            self.sessionstore.del(&session_key);
            (status::Ok, "".to_string())
        }
        else
        {
            (status::Unauthorized, "".to_string())
        }
    }
}
