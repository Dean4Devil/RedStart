//! Around Middleware providing Authentication
//! This Middleware parses incoming requests for cookies and sets cookie headers on responses as necessary

use iron::prelude::*;
use iron::{Handler, AroundMiddleware};
use iron::headers::SetCookie;
use iron::headers::Cookie as CookieHeader;

use cookie::Cookie;

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
					"auth-token" => { println!("{}", cookie.value); }
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

