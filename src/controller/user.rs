use iron::prelude::*;
use iron::response::modifiers::Status;
use iron::status;

pub struct User;

impl User
{
	pub fn new() -> User
	{
		User
	}

	impl call(&self, model: &str, req: &mut Request) -> (Status, String)
	{
		let login = Login::new();
		let logout = Logout::new();
		let setline = SetLine::new();
		let info = Info::new();

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
			"setline" =>
			{
				setline.call(req)
			},
			"info" =>
			{
				info.call(req)
			},
			_ =>
			{
				(Status(status::NotFound), "".to_string())
			},
		}
	}
}

struct Login;

impl Login
{
    pub fn new() -> Login
    {
        Login
    }

    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        (Status(status::NotImplemented), "".to_string())
    }
}

impl Logout
{
    pub fn new() -> Logout
    {
        Logout
    }

    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        (Status(status::NotImplemented), "".to_string())
    }
}

impl SetLine
{
    pub fn new() -> SetLine
    {
        SetLine
    }

    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        (Status(status::NotImplemented), "".to_string())
    }
}

impl Info
{
    pub fn new() -> SetLine
    {
        Info
    }

    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        (Status(status::NotImplemented), "".to_string())
    }
}