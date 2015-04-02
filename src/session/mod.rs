//! Session management
use iron::typemap::Key;

mod storage;

pub struct Session
{
    pub key: String,
    pub username: String,
}
impl Session
{
    pub fn new(key: String, username: String) -> Session
    {
        Session { key: key, username: username }
    }
}
impl Key for Session { type Value = String; }
impl Clone for Session
{
    fn clone(&self) -> Session
    {
        Session { key: self.key.clone(), username: self.username.clone() }
    }
}

pub struct Store
{
    engine: storage::Memory,
}
impl Store
{
    pub fn new() -> Store
    {
        Store { engine: storage::Memory::new() }
    }
}
impl Clone for Store
{
    fn clone(&self) -> Store
    {
        Store { engine: self.engine.clone() }
    }
}
impl SessionStore for Store
{
    fn get(&self, key: &String) -> Option<Session> { self.engine.get(key) }
    fn put(&self, key: &String, session: Session) { self.engine.put(key, session) }
    fn del(&self, key: &String) { self.engine.del(key) }
}

pub trait SessionStore
{
    fn get(&self, key: &String) -> Option<Session>;
    fn put(&self, key: &String, session: Session);
    fn del(&self, key: &String);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn session_gen()
    {
        let key = "testkey".to_string();
        let uname = "testusername".to_string();
        let session = Session::new(key.clone(), uname.clone());
        assert_eq!(session.key, key);
        assert_eq!(session.username, uname);
    }
}

