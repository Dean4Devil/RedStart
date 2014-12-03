use iron::prelude::*;
use iron::{BeforeMiddleware, Error};

pub struct PermCheck;

impl BeforeMiddleware for PermCheck
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        Ok(())
    }
}

#[deriving(Show)]
pub struct InsufficientPermissions;
#[deriving(Show)]
pub struct NotLoggedIn;

impl Error for InsufficientPermissions
{
    fn name(&self) -> &'static str { "InsufficientPermissions" }
}

impl Error for NotLoggedIn
{
    fn name(&self) -> &'static str { "NotLoggedIn" }
}
