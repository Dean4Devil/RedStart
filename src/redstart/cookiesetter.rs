use iron::prelude::*;
use iron::AfterMiddleware;
use iron::headers::SetCookie;

use cookie::Cookie;

use redstart::{Session, Store, SessionStore};

pub struct CookieSetter
{
    sessionstore: Store,
}

impl CookieSetter
{
    pub fn new(sessionstore: Store) -> CookieSetter
    {
        CookieSetter { sessionstore: sessionstore }
    }
}

impl AfterMiddleware for CookieSetter
{
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response>
    {
        if req.extensions.contains::<Session>()
        {
            let session_key: String = req.extensions.get_mut::<Session>().unwrap().clone();
            let cookie = Cookie::new("auth-token".to_string(), session_key);
            res.headers.set(SetCookie(vec![cookie]));
        }
        Ok(res)
    }
}
