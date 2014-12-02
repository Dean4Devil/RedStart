use iron::prelude::*;
use iron::{BeforeMiddleware, Error};

pub struct PermCheck;

impl BeforeMiddleware for PermCheck
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        Err(box NotLoggedIn as IronError)
    }
}

#[deriving(Show)]
pub struct InsufficientPermissions;
#[deriving(Show)]
pub struct NotLoggedIn;

impl Error for InsufficientPermissions
{
    fn name(&self) -> &'static str { "The request has insufficient permission for this action" }
}

impl Error for NotLoggedIn
{
    fn name(&self) -> &'static str { "This action requires an authenticated user" }
}
