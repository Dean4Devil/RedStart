use iron::prelude::*;
use iron::{Handler, AfterMiddleware};
use iron::status::{self, Status};

use controller::Reservation;

// Re-export Logger and Router so you can use redstart::Router instead of redstart::router::Router.
pub use self::logger::Logger;
pub use self::urlparser::{URLParser, URL};
pub use self::permission::PermCheck;
pub use self::configreader::ConfigReader;

mod logger;
mod urlparser;
mod permission;
mod configreader;
// End Re-export

pub struct RedStart;

impl Handler for RedStart
{
    fn handle(&self, req: &mut Request) -> IronResult<Response>
    {
        // Define some arbitrary variables. ToDo: These should be set by URLParser later on
        let controller: &str;
        let model: &str;
        println!("3");
        {
            let req_ext: &[&str] = req.extensions.get::<URL>().unwrap(); // If this panics, URLParser has a bug! :D
            // Guess what!
            controller = req_ext[0].clone();
            model = req_ext[1].clone();
        }

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
        println!("4");
        Ok(Response::new().set(status).set(body))
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
