use iron::prelude::*;
use iron::response::modifiers::Status;
use iron::status;

pub struct Reservation;

impl Reservation
{
    pub fn new() -> Reservation
    {
        Reservation
    }
    pub fn call(&self, model: &str, req: &mut Request) -> (Status, String)
    {
        let timetable = Timetable::new();
        let reservation = ReservationDisplay::new();
        // Currently statically defined, later pull from request.
        match model
        {
            "timetable" =>
            {
               timetable.call(req) 
            },
            "reservation" =>
            {
                reservation.call(req)
            },
            _ =>
            {
                (Status(status::NotFound), "".to_string())
            },
        }
    }

}

struct Timetable;

impl Timetable
{
    pub fn new() -> Timetable
    {
        Timetable
    }

    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        let body = "{'Raum137':[{'Moosbauer':[1,2,3],'Buergis':[5]},{'Krueger':[2,3],'Moosbauer':[1,5],'Noerpel':[6]},{'Buergis':[1,2,5,6],'Moosbauer':[4]},{'Krueger':[2,3],'Noerpel':[5]},{'Noerpel':[5,6],'Moosbauer':[4],'Krueger':[1,2]}],'Raum116':[{'Moosbauer':[1,2,3],'Buergis':[5]},{'Krueger':[2,3],'Moosbauer':[1,5],'Noerpel':[6]},{'Buergis':[1,2,5,6],'Moosbauer':[4]},{'Krueger':[2,3],'Noerpel':[5]},{'Noerpel':[5,6],'Moosbauer':[4],'Krueger':[1,2]}]}";
        (Status(status::Ok), body.to_string())
    }
}

struct ReservationDisplay;

impl ReservationDisplay
{
    pub fn new() -> ReservationDisplay
    {
        ReservationDisplay
    }

    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        let body = "{'Donnerstag,25.9.2014':{'Raum137':{'Krueger':[4,5,6],'Moosbauer':[1,2,3]}}}";
        (Status(status::Ok), body.to_string())
    }
}