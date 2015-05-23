/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use std::io::Read;

use serialize::json;

use iron::prelude::*;
use iron::status::{self, Status};

use api::{API, GGNet};
use session::{Session, Store, SessionStore};
use cookiesetter::CookieReq;

pub struct User
{
    api: API,
}
impl User
{
    pub fn new(api: API) -> User
    {
        User { api: api }
    }

    pub fn call(&self, model: &str, req: &mut Request) -> Response
    {
        // The Store is a Arc so no problem cloning it.
        let login = Login::new(self.api.sessions.clone());
        let logout = Logout::new(self.api.sessions.clone());
        let mut list = List::new(self.api.ggnet.clone());

        let (status, body) = match model
        {
            "login" =>
            {
                login.call(req)
            },
            "logout" =>
            {
                logout.call(req)
            },
            "list" =>
            {
                list.call(req)
            }
            _ =>
            {
                (status::NotFound, "".to_string())
            },
        };

        Response::new().set(status).set(body)
    }
}

struct Login
{
    sessionstore: Store,
}
impl Login
{
    pub fn new(sessionstore: Store) -> Login
    {
        Login { sessionstore: sessionstore }
    }
    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        let mut req_string = String::new();
        req.body.read_to_string(&mut req_string).unwrap();

        if cfg!(cfg = "debug")
        {
            println!("{}", req_string);
        }

        if  req_string == "username=testuser&password=testpass"
        {
            // Username + password "valid"
            //let mut rng = rand::thread_rnd();

            let session_key = "123456".to_string();
            //if cfg!("debug")
            //{
                println!("{}", session_key);
            //}

            let session = Session::new(session_key.clone(), "testuser".to_string());
            self.sessionstore.put(&session_key, session);
            req.extensions.insert::<CookieReq>(vec![["auth-token".to_string(), session_key]]);
            (status::Accepted, "".to_string())
        }
        else
        {
            (status::Unauthorized, "".to_string())
        }
    }
}

struct Logout
{
    sessionstore: Store,
}
impl Logout
{
    pub fn new(sessionstore: Store) -> Logout
    {
        Logout { sessionstore: sessionstore }
    }
    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        if req.extensions.contains::<Session>()
        {
            let session_key: String = req.extensions.get_mut::<Session>().unwrap().clone();
            self.sessionstore.del(&session_key);
            (status::Ok, "".to_string())
        }
        else
        {
            (status::Unauthorized, "".to_string())
        }
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
        let res_vec = self.ggnet.get_users("*");
        (status::Ok, json::encode(&res_vec).unwrap())
    }
}
