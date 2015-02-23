//! Session management

use iron::typemap::Key;

mod storage;

pub struct Session
{
	pub key: String,
}
impl Key for Session { type Value = String; }

pub struct Store<S: Storage>
{
    engine: S,
}

trait Storage
{
    fn get_from_key(&self, key: &str) -> Result<Session, ()>;
}
