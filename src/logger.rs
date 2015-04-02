use iron::prelude::*;

use iron::AfterMiddleware;

/// The logger struct
pub struct Logger
{
    path: Path,
}

impl Logger
{
    pub fn new(filename: &str) -> Logger
    {
        let path = Path::new(filename);
        Logger { path: path }
    }
    
    fn log(&self, _: &Request, _: &Response)
    { }
}

impl AfterMiddleware for Logger
{
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response>
    {
        self.log(req, &res);
        Ok(res)
    }
}
