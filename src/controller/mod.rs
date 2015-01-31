use std::collections::HashMap;

use model::Model;

use iron::prelude::*;
use iron::status;

pub use self::reservation::Reservation;

mod reservation;
