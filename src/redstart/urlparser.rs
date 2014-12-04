use iron::prelude::*;
use iron::{BeforeMiddleware, Error};

use url::{Url, SchemeData};

// Errors for the win!
#[deriving(Show)]
pub struct MalformedRequest;
#[deriving(Show)]
pub struct NotFound;

impl Error for MalformedRequest
{
    fn name(&self) -> &'static str { "MalformedRequest" }
}

impl Error for NotFound
{
    fn name(&self) -> &'static str { "NoRoute" }
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
        if(check_url(req))
        {
        	Ok(())
        }
        else
        {
        	Err(box MalformedRequest as IronError)
        }
    }
}

fn check_url(req: &mut Request) -> bool
{
	let parsed = Url::parse(req.url.to_string().as_slice()).ok().unwrap();
	let path = parsed.path();
	println!("{}\n{}", path, path.unwrap().len());
	if(path.unwrap()[0] != "".to_string() || path.unwrap().len() != 1)
	{
		return false;
	}

	let mut found = false;
    // This one returns an Option
    let mut query: Vec<(String, String)>;
    query = match parsed.query_pairs()
    {
        Some(query) => { query },
        None => { vec![("r".to_string(), "".to_string())] },
    };

	let get = String::from_str("r");

    // ToDo: This parses the whole query string, no matter if that is actually necessary. We could
    // change that, or parse the whole query string and save its values so the handler does not
    // have to parse it.
    for x in query.iter()
	{
		match x
		{
			&(ref get, ref value) =>
            { 
                found = value.contains("/");
            },
			// &(_, _) => { false },
		};
	}
    return found;
}
