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
    pub fn call(&self, req: &mut Request) -> (Status, String)
    {
        let timetable = Timetable::new();
        // Currently statically defined, later pull from request.
        let model = "timetable";
        match model
        {
            "timetable" =>
            {
               timetable.call(req) 
            }
            _ =>
            {
                (Status(status::NotFound), "".to_string())
            }
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
        (Status(status::Ok), "Hello world from Timetable!".to_string())
    }
}
