use iron::prelude::*;
use iron::BeforeMiddleware;

/// The actual Router struct
pub struct Router;

/// Make Router a BeforeMiddleware
impl BeforeMiddleware for Router
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        // Currently do precisely nothing
        Ok(())
    }
}
