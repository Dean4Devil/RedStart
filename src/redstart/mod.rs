use hyper::server::{Request, Response};
use hyper::header::common::ContentLength;
use hyper::{Get, Post, uri, status};

pub fn serve(mut req: Request, mut res: Response)
{
    println!("{}", req.uri);
    match req.uri
    {
        // Match the request URL. If its an AbsolutePath, match it further.
        uri::AbsolutePath(ref path) => match(&req.method, path.as_slice())
        {
            (&Get, "/") =>
            {
               let out = b"You GET requested '/'";
               
               res.headers_mut().set(ContentLength(out.len()));
               let mut res = res.start().unwrap();
               res.write(out).unwrap();
               res.end().unwrap(); 
               return;
            },
            _ =>
            {
                *res.status_mut() = status::NotFound;
                res.start().and_then(|res| res.end());
                return; 
            }
        },
        _ =>
        {
            return;
        }
    }
    let mut res = res.start().unwrap();
    res.write(b"Hello World!").unwrap();
    res.end().unwrap();
}
