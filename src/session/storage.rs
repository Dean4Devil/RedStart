/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use super::{Session, SessionStore};

/// Pure in-memory session storage
pub struct Memory
{
    // The storeage is a read-write locked hashmap
    store: Arc<RwLock<HashMap<String, Session>>>,
}

impl Memory
{
    pub fn new() -> Memory
    {
        Memory { store: Arc::new(RwLock::new(HashMap::<String, Session>::new())) }
    }
}

// Memory implements SessionStore to guarantee to the parent Store struct that it has a few given
// functions.
impl SessionStore for Memory
{
    fn put(&self, key: &String, session: Session)
    {
        // Aquire a write lock for the shared HashMap and insert the key-value pair
        self.store.write().unwrap().insert(key.to_string().clone(), session);
    }
    
    fn get(&self, key: &String) -> Option<Session>
    {
        // Aquire a read lock for the shared HashMap and clone() the value out
        match self.store.read().unwrap().get(key)
        {
            None => return None,
            Some(val_ref) =>
            {
                let val: Session = val_ref.clone();
                return Some(val);
            }
        }
    }

    fn del(&self, key: &String)
    {
        self.store.write().unwrap().remove(key);
    }
}

impl Clone for Memory
{
    fn clone(&self) -> Memory
    {
        Memory { store: self.store.clone() }
    }
}


#[cfg(test)]
pub struct Null;

#[cfg(test)]
impl SessionStore for Null
{
    fn put(&self, key: &String, session: Session)
    {
        // Don't do anything, but most importantly: Do not fail.
    }

    fn get(&self, key: &String) -> Option<Session>
    {
        // Never return `None`
        return Some(Session::new(key.clone(), "testuser".to_string()));
    }

    fn del(&self, key: &String)
    {
        // Don't do anything, but most importantly: Do not fail.
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use session::{Session, SessionStore};

    #[test]
    fn put_get()
    {
        let key = "testkey".to_string();
        let session = Session::new(key.clone(), "testusername".to_string());
        let store = Memory::new();
        store.put(&key, session.clone());
        let session2 = store.get(&key).unwrap();
        assert_eq!(session.username, session2.username);
    }

    #[test]
    fn put_del_get()
    {
        let key = "testkey".to_string();
        let session = Session::new(key.clone(), "testusername".to_string());
        let store = Memory::new();
        store.put(&key, session.clone());
        store.del(&key);
        let session2 = store.get(&key);
        assert_eq!(session2, None)
    }

    #[test]
    fn get()
    {
        let key = "testkey".to_string();
        let store = Memory::new();
        let session = store.get(&key);
        assert_eq!(session, None)
    }
}

