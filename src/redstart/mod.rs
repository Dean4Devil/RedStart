use iron::prelude::*;
use iron::Handler;
use iron::response::modifiers::{Status, Body};
use iron::status;

use controller::Reservation;

// Re-export Logger and Router so you can use redstart::Router instead of redstart::router::Router.
pub use self::logger::Logger;
pub use self::urlparser::URLParser;
pub use self::permission::PermCheck;

mod logger;
mod urlparser;
mod permission;
// End Re-export

pub struct RedStart;

impl Handler for RedStart
{
    fn call(&self, req: &mut Request) -> IronResult<Response>
    {
        // Define some arbitrary variables. ToDo: These should be set by URLParser later on
        let controller: &str;
        let model: &str;

        {
            let req_ext = req.extensions.get::<URLParser, &'static [&'static str]>().unwrap(); // If this panics, URLParser has a bug! :D
            controller = req_ext[0].clone();
            model = req_ext[1].clone();
        }

        let status: Status;
        let body: &str;

        let reservation = Reservation::new();

        let (status, body) = match controller
        {
            "reservation" => { reservation.call(model, req) },
            _ => 
            {
                (Status(status::NotFound), "".to_string())
            },
        };

        Ok(Response::new().set(status).set(Body(body)))
    }

    fn catch(&self, _: &mut Request, err: IronError) -> (Response, IronResult<()>)
    {
        // Its definetely *not* pretty. But it works.
        match err.name()
        {
            "NotLoggedIn" => { (Response::new().set(Status(status::Unauthorized)), Ok(())) },
            "InsufficientPermissions" => { (Response::new().set(Status(status::Forbidden)), Ok(())) },
            "NoRoute" => { (Response::new().set(Status(status::NotFound)), Ok(())) },
            "MalformedRequest" => { (Response::new().set(Status(status::BadRequest)), Ok(())) },
            _ => { (Response::new().set(Status(status::InternalServerError)), Ok(())) },
        }
    }
}
