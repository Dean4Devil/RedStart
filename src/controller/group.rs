/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use std::io::Read;

use iron::prelude::*;
use iron::status::{self, Status};

use api::{API, GGNet};

pub struct Group
{
    api: API,
}
impl Group
{
    pub fn new(api: API) -> Group
    {
        Group { api: api }
    }

    pub fn call(&self, model: &str, req: &mut Request) -> Response
    {
        let body: Box<Read + Send>;
        let (status, body) = match model
        {
            "list" =>
            {
                (status::NotImplemented, "".to_string())
            },
            _ =>
            {
                (status::NotFound, "".to_string())
            },
        };

        Response::new().set(status).set(body)
    }
}
