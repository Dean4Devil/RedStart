use iron::prelude::*;
use iron::BeforeMiddleware;

pub struct PermCheck;

impl BeforeMiddleware for PermCheck
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        Ok(())
    }
}
