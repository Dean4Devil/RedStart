/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use serialize::json;

use iron::prelude::*;
use iron::status::{self, Status};

use api::{API, GGNet};

use redstart::Controller;

pub struct Group
{
    api: API,
}
impl Group
{
    pub fn new(api: &API) -> Group
    {
        Group { api: api.clone() }
    }

}

impl Controller for Group
{
    fn name(&self) -> &'static str
    {
        "group"
    }

    fn call(&self, model: Option<String>, req: &mut Request) -> Response
    {
        let mut list = List::new(self.api.ggnet.clone());

        let (status, body) = match model
        {
            Some(e) =>
            {
                match e.as_ref()
                {
                    "list" => list.call(req),
                    _ => (status::NotFound, "".to_string())
                }
            },
            _ =>
            {
                (status::NotFound, "".to_string())
            },
        };

        Response::new().set(status).set(body)
    }
}

struct List
{
    ggnet: GGNet,
}

impl List
{
    pub fn new(ggnet: GGNet) -> List
    {
        List { ggnet: ggnet }
    }

    pub fn call(&mut self, _: &mut Request) -> (Status, String)
    {
        let res_vec = self.ggnet.get_groups("*");
        (status::Ok, json::encode(&res_vec).unwrap())
    }
}
