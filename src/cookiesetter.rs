use iron::prelude::*;
use iron::AfterMiddleware;
use iron::headers::SetCookie;
use iron::typemap::Key;

use cookie::Cookie;

use session::{Session, Store, SessionStore};

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
        if req.extensions.contains::<CookieReq>()
        {
            let cookievalvec: Vec<[String; 2]> = req.extensions.remove::<CookieReq>().unwrap();
            let cookies: Vec<Cookie> = cookievalvec.into_iter().map(|[x, y]| Cookie::new(x,y)).collect();
            res.headers.set(SetCookie(cookies));
        }
        Ok(res)
    }
}

pub struct CookieReq;
impl Key for CookieReq { type Value = Vec<[String; 2]>; }
