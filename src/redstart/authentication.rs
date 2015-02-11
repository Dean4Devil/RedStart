//! Around Middleware providing Authentication
//! This Middleware parses incoming requests for cookies and sets cookie headers on responses as necessary

use iron::prelude::*;
use iron::{Handler, AroundMiddleware};
use iron::headers::SetCookie;
use iron::headers::Cookie as CookieHeader;
use iron::status::{Status, self};

use cookie::Cookie;

use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct AuthTimeout(String);
#[derive(Debug)]
pub struct AuthError(String);

impl Error for AuthTimeout
{
	fn description(&self) -> &'static str { "AuthTimeout" }
}

impl fmt::Display for AuthTimeout
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Error for AuthError
{
	fn description(&self) -> &'static str { "AuthError" }
}

impl fmt::Display for AuthError
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

pub struct AuthHandler<H> { handler: H, }
pub struct AuthMiddleware;

impl<H: Handler> Handler for AuthHandler<H>
{
	fn handle(&self, req: &mut Request) -> IronResult<Response>
	{
		if req.headers.has::<CookieHeader>() { 
			println!("Request comes with Cookies!! *nom*");
			let cookies = req.headers.get::<CookieHeader>().unwrap();
			for cookie in cookies.iter()
			{
				// Match over every cookie in the request
				match cookie.name.as_slice()
				{
					"auth-token" => {
						// Request has the Auth token set. Check if its good and set login if so.
						match cookie.value.as_slice() {
							"UUXzTqbFRdzbr79" =>
							{
								println!("Request has a valid token!");
							}
							"ixxKo5obDmees6o" =>
							{
								println!("Request has an expired token!");
							}
							_ =>
							{
								println!("Request has an invalid token!");
							}
						}
					}
					_ => {}
				}
			}
		}
		let mut res = self.handler.handle(req).unwrap();
		let mut cookie = Cookie::new("test".to_string(), "succeed".to_string());
		res.headers.set(SetCookie(vec![cookie]));
		return Ok(res);
	}
}

impl AuthMiddleware
{
	pub fn new() -> AuthMiddleware { AuthMiddleware }
}

impl AroundMiddleware for AuthMiddleware
{
	fn around(self, handler: Box<Handler>) -> Box<Handler>
	{
		Box::new(AuthHandler{
					handler: handler,
				}) as Box<Handler>
	}
}

