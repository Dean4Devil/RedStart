//! Around Middleware providing Authentication
//! This Middleware parses incoming requests for cookies and sets cookie headers on responses as necessary

use iron::prelude::*;
use iron::{Handler, AroundMiddleware};


pub struct AuthHandler<H> { handler: H, }
pub struct AuthMiddleware;

impl<H: Handler> Handler for AuthHandler<H>
{
	fn handle(&self, req: &mut Request) -> IronResult<Response>
	{
		self.handler.handle(req)
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

