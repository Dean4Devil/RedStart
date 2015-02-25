use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use super::{Session, SessionStore};

// Pure in-memory session storage
struct Memory
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
        match self.store.read().unwrap().get_mut(key)
        {
            None => return None,
            Some(&session) => return session.clone(),
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

