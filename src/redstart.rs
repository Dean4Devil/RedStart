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
}

impl RedStart
{
    pub fn new(api: API) -> RedStart
    {
        RedStart { api: api }
    }
}

#[allow(unused_variables)]
impl Handler for RedStart
{
    fn handle(&self, req: &mut Request) -> IronResult<Response>
    {
        let reservation = Reservation::new();
        let user = User::new(self.api.clone());

        let ext_url: [String; 2] = req.extensions.remove::<URL>().unwrap(); // If this panics, URLParser has a bug! :D
        let mut res: Response = match ext_url[0].as_slice()
        {
            "reservation" => { reservation.call(ext_url[1].as_slice(), req) },
            "user" => { user.call(ext_url[1].as_slice(), req) },
            _ =>
            {
                let body: Box<Reader + Send> = Box::new("".as_bytes());
                Response::new().set(status::NotFound).set(body)
            },
        };

        let mime: mime::Mime = "application/json".parse().unwrap();
        res.headers.set(ContentType(mime));
        Ok(res)
    }
}

