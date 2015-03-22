//! This module contains functions that provide the same functionality as ggnpwcheck.inc

use ldap;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct GGNet
{
    // To keep the API as thread-safe as possible we hide the LDAP connection behind a Mutex.
    // TODO: Check performance and upgrade to several connections / a connection pool if necessary
    ldap: Arc<Mutex<ldap::LDAP>>,
}

impl GGNet
{
    pub fn new() -> GGNet
    {
        let mut ldap = ldap::LDAP::new();
        // TODO Replace this with some proper error checking
        if !ldap.initialize("ldap://localhost:3890") { panic!("GGNet's LDAP Initialization failed!!") }

        ldap.set_option();

        if !ldap.simple_bind("cn=admin,dc=ad,dc=ggnet", "DidRPwfLDAP!") { panic!("GGNet's LDAP binding failed!") }

        GGNet { ldap: Arc::new(Mutex::new(ldap)) }
    }

    pub fn user_exists(&mut self, username: &str) -> bool
    {
        // We grab the lock with (*self.ldap) because Arc acts like a pointer
        let mut ld = (*self.ldap).lock().unwrap();

        // TODO: Change this to only request the uidNumber when bug https://github.com/Dean4Devil/rust-ldap/issues/1 is fixed.
        let result = ld.search("ou=Benutzer,dc=ad,dc=ggnet", 1, "(&(cn=testuser)(objectClass=person))", &["cn", "uidNumber"], 0);
        if result.is_none() { return false; }

        let mut entry = result.unwrap().first_entry(&mut ld);
        entry.is_some()
    }

    pub fn group_exists(&mut self, groupname: &str) -> bool
    {
        // We grab the lock with (*self.ldap) because Arc acts like a pointer
        let mut ld = (*self.ldap).lock().unwrap();

        // TODO: Change this to only request the uidNumber when bug https://github.com/Dean4Devil/rust-ldap/issues/1 is fixed.
        let result = ld.search("ou=Gruppen,dc=ad,dc=ggnet", 1, "(objectClass=gruppe)", &["cn", "displayName", "gidNumber"], 0);

        if result.is_none() { return false; }

        // The result of this statement will be returned
        result.unwrap().count_entries(&mut ld) < 1
    }

    pub fn get_users(&mut self, filter: &str) -> Vec<String>
    {
        // Grab the lock
        let mut ld = (*self.ldap).lock().unwrap();

        // TODO: Change this to only request the uidNumber when bug https://github.com/Dean4Devil/rust-ldap/issues/1 is fixed.
        let mut result_o = ld.search("ou=Benutzer,dc=ad,dc=ggnet", 1, "(&(cn=*)(objectClass=person))", &["cn", "uidNumber"], 0);

        if result_o.is_none() { return Vec::new(); }
        let mut result = result_o.unwrap();

        let mut users: Vec<String> = Vec::new();
        let mut entry_o = result.first_entry(&mut ld);
        let mut i: i32 = 0;
        loop
        {
            if entry_o.is_none() { break; }
            let mut entry = entry_o.unwrap();
            if entry.is_null() { break; }
            
            users.push(entry.get_values(&mut ld, "cn"));
            
            entry_o = entry.next_entry(&mut ld);
        }

        users
    }

    pub fn get_groups(&mut self, filter: &str) -> Vec<String>
    {
        // Grab the lock
        let mut ld = (*self.ldap).lock().unwrap();

        // TODO: Change this to only request the uidNumber when bug https://github.com/Dean4Devil/rust-ldap/issues/1 is fixed.
        let mut result_o = ld.search("ou=Gruppen,dc=ad,dc=ggnet", 1, "(objectClass=gruppe)", &["cn", "gidNumber"], 0);

        if result_o.is_none() { return Vec::new(); }
        let mut result = result_o.unwrap();

        let mut  groups: Vec<String> = Vec::new();
        let mut entry_o = result.first_entry(&mut ld);
        loop
        {
            if entry_o.is_none() { break; }
            let mut entry = entry_o.unwrap();
            
            groups.push(entry.get_values(&mut ld, "cn"));
            
            entry_o = entry.next_entry(&mut ld);
        }

        groups
    }

    pub fn get_group_members(&mut self, groupname: &str) // -> &[String]
    {

    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn testuser_exists()
    {
        // Has to be mutable!
        let mut ggconn = GGNet::new();

        assert!(ggconn.user_exists("testuser"));
    }

    #[test]
    fn listusers()
    {
        let mut ggconn = GGNet::new();

        assert_eq!(ggconn.get_users(""), vec!["testuser".to_string(), "testuse2".to_string()]);
    }

    fn testgroup_exists()
    {
        // Has to be mutable!
        let mut ggconn = GGNet::new();

        assert!(ggconn.group_exists("testgroup"));
    }
}

