use iron::prelude::*;
use iron::{BeforeMiddleware, Error};

// Re-export recognizer::Params as router::Params
pub use recognizer::Params;

// Errors for the win!
#[deriving(Show)]
pub struct MalformedRequest;
#[deriving(Show)]
pub struct NotFound;

impl Error for MalformedRequest
{
    fn name(&self) -> &'static str { "Malformed Request" }
}

impl Error for NotFound
{
    fn name(&self) -> &'static str { "No route" }
}

// The actual URLParser struct
pub struct URLParser;

impl URLParser
{
    
}

// Make URLParser a BeforeMiddleware
impl BeforeMiddleware for URLParser
{
    fn before(&self, req: &mut Request) -> IronResult<()>
    {
        // Currently do precisely nothing
        Ok(())
    }
}


