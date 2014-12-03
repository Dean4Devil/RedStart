use iron::prelude::*;
use iron::{BeforeMiddleware, Error};

// Re-export recognizer::Params as router::Params
pub use recognizer::Params;

// Errors for the win!
#[deriving(Show)]
pub struct MalformedRequest;
#[deriving(Show)]
pub struct NoRoute;

impl Error for MalformedRequest
{
    fn name(&self) -> &'static str { "Malformed Request" }
}

impl Error for NoRoute
{
    fn name(&self) -> &'static str { "No route" }
}

/// The actual Router struct
pub struct Router;

impl Router
{
    
}

/// Make Router a BeforeMiddleware
impl BeforeMiddleware for Router
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        // Currently do precisely nothing
        Ok(())
    }
}


