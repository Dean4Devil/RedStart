/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

pub struct Storage;

impl Storage
{
    pub fn new() -> Storage {
        Storage
    }

    pub fn lookup(&self, groupname: &str) -> Vec<String>
    {
        vec!["some.permission".to_string(), "someother.perm".to_string(), "some.per".to_string()]
    }
}
