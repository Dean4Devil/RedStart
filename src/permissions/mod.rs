/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use self::storage::Storage;

pub use self::errors::*;

// Different types of storage (memory, flatfile, db)
mod storage;
mod errors;

#[derive(Debug)]
pub struct Group
{
    // The name of the group
    name: String,
    // The permissions attached to this group/role
    // TODO: Find a non-stringy way to work with permissions maybe?
    // TODO: Is a Vector the best solution?
    permissions: Vec<String>,
}

impl Group
{
    pub fn new(name: String) -> Group
    {
        // Get this groups permissions
        let storage = Storage::new();
        let permissions = storage.lookup(name.as_ref());

        Group { name: name, permissions: permissions }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn group_gen()
    {
        let group = Group::new("testgroup".to_string());

        assert_eq!(group, Group { name: "testgroup".to_string(), permissions: vec!["some.permission".to_string(), "someother.perm".to_string(), "some.per".to_string()] })
    }
}
