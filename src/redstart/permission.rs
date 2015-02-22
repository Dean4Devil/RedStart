use iron::prelude::*;
use iron::BeforeMiddleware;

use std::error::Error;
use std::fmt::{self, Debug};

pub struct PermCheck;

impl BeforeMiddleware for PermCheck
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        // Todo: Move this Middleware's functionality into the Handler.
        Ok(())
    }
}

#[derive(Debug)]
pub struct InsufficientPermissions;
#[derive(Debug)]
pub struct NotLoggedIn;

impl Error for InsufficientPermissions
{
    fn description(&self) -> &'static str { "InsufficientPermissions" }
}

impl fmt::Display for InsufficientPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotLoggedIn
{
    fn description(&self) -> &'static str { "NotLoggedIn" }
}

impl fmt::Display for NotLoggedIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
