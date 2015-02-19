//! Around Middleware providing Authentication
//! This Middleware parses incoming requests for cookies and sets cookie headers on responses as necessary


pub mod Authentication
{
	use iron::prelude::*;
	use iron::{Handler, AroundMiddleware};
	use iron::headers::SetCookie;
	use iron::headers::Cookie as CookieHeader;
	use iron::status::{Status, self};

	use cookie::Cookie;

	use std::error::Error;
	use std::fmt::{self, Debug};

	#[derive(Debug)]
	pub struct AuthTimeout;
	#[derive(Debug)]
	pub struct AuthError;

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

	pub struct User
	{
		pub name: String,
	}

	pub fn parse_from_token(token: &str) -> IronResult<User>
	{
		// TODO Remove Placeholders
		match token
		{
			// Placeholder correct token
			"UUXzTqbFRdzbr79" =>
			{
				return Ok(User { name: "testuser".to_string() })
			},
			// Placeholder outdated token
			"ixxKo5obDmees6o" =>
			{
				// TODO This should return a specific error.
				return Err(IronError::new(AuthTimeout, status::Forbidden));
			}
            // Placeholder invalid token
			_ =>
			{
				return Err(IronError::new(AuthError, status::Forbidden));
			}
		}
	}

	pub mod API
	{
	}

}
