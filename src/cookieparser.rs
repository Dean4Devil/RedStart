use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::status;
use iron::headers::Cookie as CookieHeader;

use authentication::Authentication::AuthError;
use session::{Session, Store, SessionStore};

/// The CookieParser looks for Cookies in incoming requests and parses them by given rules.
pub struct CookieParser
{
    sessionstore: Store,
}

impl CookieParser
{
    pub fn new(sessionstore: Store) -> CookieParser
    {
        CookieParser { sessionstore: sessionstore }
    }
}

// BeforeMiddleware means that CookieParser gets called before the main redstart Handler.
impl BeforeMiddleware for CookieParser
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        // Check if there are cookies in the request
        if req.headers.has::<CookieHeader>() {
            let cookies = req.headers.get::<CookieHeader>().unwrap();
            for cookie in cookies.iter()
            {
                let result: IronResult<()> = match cookie.name.as_slice()
                {
                    "auth-token" =>
                    {
                        let session = self.sessionstore.get(&cookie.value);
                        if session.is_some()
                        {
                            println!("Auth-Token is valid!");
                            req.extensions.insert::<Session>(session.unwrap().key);
                            Ok(())
                        }
                        else
                        {
                            println!("Auth-token is set but is not valid!");
                            return Err(IronError::new(AuthError, status::Unauthorized));
                        }
                    },

                    // Anything *except* "auth-token"
                    _ =>
                    {
                        Ok(())
                    }
                };

                if result.is_err()
                {
                    return result;
                }
            }
        }
        
        // Even if not the request is still valid, so pass on.
        Ok(())
    }
}

