/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::status;
use iron::headers::Cookie as CookieHeader;

use api::API;
use authentication::Authentication::AuthError;
use session::{Session, Store, SessionStore};

/// The CookieParser looks for Cookies in incoming requests and parses them by given rules.
pub struct CookieParser
{
    sessionstore: Store,
}

impl CookieParser
{
    pub fn new(api: &API) -> CookieParser
    {
        CookieParser { sessionstore: api.sessions.clone() }
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
                let result: IronResult<()> = match cookie.name.as_ref()
                {
                    "auth-token" =>
                    {
                        let session = self.sessionstore.get(&cookie.value);
                        if session.is_some()
                        {
                            dbgprint!("Auth-Token is valid!");
                            req.extensions.insert::<Session>(session.unwrap().key);
                            Ok(())
                        }
                        else
                        {
                            dbgprint!("Auth-token is set but is not valid!");
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

