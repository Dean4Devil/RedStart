/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use iron::prelude::*;
use iron::status;

use api::API;
use redstart::Controller;

pub struct Reservation;

impl Reservation
{
    pub fn new(api: &API) -> Reservation
    {
        Reservation
    }
}

impl Controller for Reservation
{
    fn name(&self) -> &'static str
    {
        "reservation"
    }

    fn call(&self, model: Option<String>, req: &mut Request) -> Response
    {
        let timetable = Timetable::new();
        let reservation = ReservationDisplay::new();
        // Currently statically defined, later pull from request.
        let (status, body) = match model
        {
            Some(e) =>
            {
                match e.as_ref()
                {
                    "timetable" =>
                    {
                       timetable.call(req) 
                    },
                    "reservation" =>
                    {
                        reservation.call(req)
                    },
                    _ => (status::NotFound, "".to_string())
                }
            }
            _ =>
            {
                (status::NotFound, "".to_string())
            },
        };
        let res = Response::new();
        res.set(status).set(body)
    }

}

struct Timetable;
impl Timetable
{
    pub fn new() -> Timetable
    {
        Timetable
    }

    pub fn call(&self, _: &mut Request) -> (status::Status, String)
    {
        let body = "{'Raum137':[{'Moosbauer':[1,2,3],'Buergis':[5]},{'Krueger':[2,3],'Moosbauer':[1,5],'Noerpel':[6]},{'Buergis':[1,2,5,6],'Moosbauer':[4]},{'Krueger':[2,3],'Noerpel':[5]},{'Noerpel':[5,6],'Moosbauer':[4],'Krueger':[1,2]}],'Raum116':[{'Moosbauer':[1,2,3],'Buergis':[5]},{'Krueger':[2,3],'Moosbauer':[1,5],'Noerpel':[6]},{'Buergis':[1,2,5,6],'Moosbauer':[4]},{'Krueger':[2,3],'Noerpel':[5]},{'Noerpel':[5,6],'Moosbauer':[4],'Krueger':[1,2]}]}";
        (status::Ok, body.to_string())
    }
}

struct ReservationDisplay;
impl ReservationDisplay
{
    pub fn new() -> ReservationDisplay
    {
        ReservationDisplay
    }

    pub fn call(&self, _: &mut Request) -> (status::Status, String)
    {
        let body = "{'Donnerstag,25.9.2014':{'Raum137':{'Krueger':[4,5,6],'Moosbauer':[1,2,3]}}}";
        (status::Ok, body.to_string())
    }
}
