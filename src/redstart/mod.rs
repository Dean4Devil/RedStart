use hyper::server::{Request, Response};
use hyper::header::common::ContentLength;

pub fn serve(mut req: Request, mut res: Response)
{
    println!("{}", req.uri);
    let mut res = res.start().unwrap();
    res.write(b"Hello World!").unwrap();
    res.end().unwrap();
}
