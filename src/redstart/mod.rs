use iron::prelude::*;
use iron::Handler;
use iron::response::modifiers::{Status, Body};
use iron::status;

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
    fn call(&self, _: &mut Request) -> IronResult<Response>
    {
        Ok(Response::new().set(Status(status::Ok)).set(Body("Hello world!\n")))
    }

    fn catch(&self, _: &mut Request, err: IronError) -> (Response, IronResult<()>)
    {
        println!("{}\n", err);

        (Response::new()
            .set(Status(status::InternalServerError))
            .set(Body(format!("I encountered Error {} which is not handled!", err))),
         Ok(()))
    }
}

