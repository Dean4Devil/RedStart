use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::typemap::Key;
use iron::headers::Cookie as CookieHeader;

use cookie::Cookie;

use redstart::Auth;
use redstart::Session;

pub struct CookieParser;

impl BeforeMiddleware for CookieParser
{
	fn before(&self, req: &mut Request) -> IronResult<()>
	{
		// Check if there are cookies in the request
		if req.headers.has::<CookieHeader>() {
			println!("Request comes with Cookies!! *nom*");
			let cookies = req.headers.get::<CookieHeader>().unwrap();
			for cookie in cookies.iter()
			{
				let result: IronResult<()> = match cookie.name.as_slice()
				{
					"auth-token" =>
					{
						let session_res = Auth::parse_from_token(cookie.value.as_slice());
						if session_res.is_err()
						{
							println!("Auth-Token is not valid!");
							Err(session_res.err().unwrap())
						}
						else
						{
							println!("Auth-Token is valid!");
							req.extensions.insert::<Session>(session_res.unwrap().key);
							Ok(())
						}
					},


					_ =>
					{
						println!("Got unknown cookie: {}", cookie);
						Ok(())
					}
				};

				if result.is_err()
				{
					println!("Cookie parsing errored!!");
					return result;
				}
			}
		}
		
		// Even if not the request is still valid, so pass on.
		Ok(())
	}
}
