use iron::prelude::*;
use iron::{Handler, AfterMiddleware};
use iron::status::{self, Status};

use hyper::header::ContentType;
use hyper::mime;

use controller::Reservation;

// Re-export Logger and Router so you can use redstart::Router instead of redstart::router::Router.
pub use self::logger::Logger;
pub use self::urlparser::{URLParser, URL};
pub use self::permission::PermCheck;
pub use self::configreader::ConfigReader;
pub use self::authentication::AuthMiddleware;

mod logger;
mod urlparser;
mod permission;
mod configreader;
mod authentication;
// End Re-export

pub struct RedStart;

impl Handler for RedStart
{
    fn handle(&self, req: &mut Request) -> IronResult<Response>
    {
        let controller_string: String;
        let model_string: String;
        {
            let ext_ref: &mut [String] = req.extensions.get_mut::<URL>().unwrap(); // If this panics, URLParser has a bug! :D
            controller_string = ext_ref[0].clone();
            model_string = ext_ref[1].clone();
        }
        let controller: &str = controller_string.as_slice();
        let model: &str = model_string.as_slice();

        let status: Status;
        let body: Box<Reader + Send>;

        let reservation = Reservation::new();

        let (status, body) = match controller
        {
            "reservation" => { reservation.call(model, req) },
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

pub struct RedStartCatch;

impl AfterMiddleware for RedStartCatch
{
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response>
    {
        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response>
    {
        // Its definetely *not* pretty. But it works.
        match err.error.description()
        {
            "NotLoggedIn" => { Ok(Response::new().set(status::Unauthorized)) },
            "InsufficientPermissions" => { Ok(Response::new().set(status::Forbidden)) },
            "NoRoute" => { Ok(Response::new().set(status::NotFound)) },
            "MalformedRequest" => { Ok(Response::new().set(status::BadRequest)) },
            _ => { Ok(Response::new().set(status::InternalServerError)) },
        }
    }
}
