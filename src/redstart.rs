use std::io::Read;
use std::convert;

use iron::prelude::*;
use iron::Handler;
use iron::status;

use hyper::header::ContentType;
use hyper::mime;

use cookie::Cookie;

use controller::Reservation;
use controller::User;

use api::API;
use session::Store;
use urlparser::URL;
use cookiesetter::CookieReq;

/// The main Handler Struct
pub struct RedStart
{
    api: API,
    reservation: Reservation,
    user: User,
}

impl RedStart
{
    pub fn new(api: &API) -> RedStart
    {
        let reservation = Reservation::new();
        let user = User::new(api.clone());

        RedStart
        {
            api: api.clone(),
            reservation: reservation,
            user: user,
        }
    }
}

#[allow(unused_variables)]
impl Handler for RedStart
{
    fn handle(&self, req: &mut Request) -> IronResult<Response>
    {
        let ext_url: [String; 2] = req.extensions.remove::<URL>().unwrap(); // If this panics, URLParser has a bug! :D
        let mut res: Response = match ext_url[0].as_ref()
        {
            "reservation" => { self.reservation.call(ext_url[1].as_ref(), req) },
            "user" => { self.user.call(ext_url[1].as_ref(), req) },
            _ =>
            {
                let body: Box<Read + Send> = Box::new("".as_bytes());
                Response::new().set(status::NotFound).set(body)
            },
        };

        let mime: mime::Mime = "application/json".parse().unwrap();
        res.headers.set(ContentType(mime));
        Ok(res)
    }
}

