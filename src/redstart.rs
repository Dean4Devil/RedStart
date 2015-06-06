/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use std::io::Read;

use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;

use hyper::header::ContentType;
use hyper::mime;

use api::API;
use urlparser::URL;

/// The main Handler Struct
pub struct RedStart
{
    controller: HashMap<&'static str, Box<Controller + Sync + Send>>,
}

impl RedStart
{
    pub fn new(api: &API) -> RedStart
    {
        let controller = HashMap::new();

        RedStart
        {
            controller: controller,
        }
    }

    pub fn add_controller(&mut self, model: Box<Controller + Sync + Send>)
    {
        self.controller.insert((*model).name(), model);
    }

    pub fn finish(&mut self)
    {
        self.controller.shrink_to_fit();
    }
}

#[allow(unused_variables)]
impl Handler for RedStart
{
    fn handle(&self, req: &mut Request) -> IronResult<Response>
    {
        let (collection, element, field) = req.extensions.remove::<URL>().unwrap();

        let key: &str = collection.as_ref();
        let resource = self.controller.get(key);

        let mut res: Response = match resource
        {
            Some(e) =>
            {
                (*e).call(element, req)
            },
            None =>
            {
                let body: Box<Read + Send> = Box::new("".as_bytes());
                Response::new().set(status::NotFound).set(body)
            },
        };

        let mime: mime::Mime = "application/json".parse().unwrap();
        res.headers.set(ContentType(mime));
        Ok(res)
    }
}

pub trait Controller
{
    fn name(&self) -> &'static str;
    fn call(&self, model: Option<String>, req: &mut Request) -> Response;
}
