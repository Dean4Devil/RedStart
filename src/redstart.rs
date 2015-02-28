use iron::prelude::*;
use iron::Handler;
use iron::status::{self, Status};

use hyper::header::ContentType;
use hyper::mime;

use controller::Reservation;
use controller::User;

use session::{Session, Store};
use urlparser::URL;

pub struct RedStart
{
    sessionstore: Store,
}

impl RedStart
{
    pub fn new(sessionstore: Store) -> RedStart
    {
        RedStart { sessionstore: sessionstore }
    }
}

#[allow(unused_variables)]
impl Handler for RedStart
{
    fn handle(&self, req: &mut Request) -> IronResult<Response>
    {
        let controller_string: String;
        let model_string: String;
        {
            let ext_url: &mut [String] = req.extensions.get_mut::<URL>().unwrap(); // If this panics, URLParser has a bug! :D
            controller_string = ext_url[0].clone();
            model_string = ext_url[1].clone();
        }
        let controller: &str = controller_string.as_slice();
        let model: &str = model_string.as_slice();

        let session_key: Option<String>;
        if req.extensions.contains::<Session>()
        {
            let ext_session: &mut String = req.extensions.get_mut::<Session>().unwrap();
            session_key = Some(ext_session.clone());
        }
        else
        {
            session_key = None;
        }

        let reservation = Reservation::new();
        let user = User::new(self.sessionstore.clone());

        let status: Status;
        let body: Box<Reader + Send>;
        let (status, body) = match controller
        {
            "reservation" => { reservation.call(model, req) },
            "user" => { user.call(model, req) },
            _ =>
            {
                (status::NotFound, "".to_string())
            },
        };

        let mime: mime::Mime = "application/json".parse().unwrap();
        let mut res = Response::new();
        res.headers.set(ContentType(mime));
        Ok(res.set(status).set(body))
    }
}

