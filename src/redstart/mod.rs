use iron::prelude::*;

use iron::response::modifiers::{Status, Body};
use iron::status;

pub fn serve(req: &mut Request) -> IronResult<Response>
{
    println!("{}", req);
    Ok(Response::new().set(Status(status::Ok))
        .set(Body("Hello world!")))
}
