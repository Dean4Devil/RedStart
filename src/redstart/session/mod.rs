//! Session management
use iron::typemap::Key;

mod storage;

pub struct Session
{
	pub key: String,
}
impl Key for Session { type Value = String; }
impl Clone for Session
{
    fn clone(&self) -> Session
    {
        Session { key: self.key.clone() }
    }
}

pub struct Store<S: SessionStore>
{
    engine: S,
}

trait SessionStore
{
    fn get(&self, key: &String) -> Option<Session>;
    fn put(&self, key: &String, session: Session);
    fn del(&self, key: &String);
}
