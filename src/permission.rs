/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub struct InsufficientPermissions;
#[derive(Debug)]
pub struct NotLoggedIn;

impl Error for InsufficientPermissions
{
    fn description(&self) -> &'static str { "InsufficientPermissions" }
}

impl fmt::Display for InsufficientPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotLoggedIn
{
    fn description(&self) -> &'static str { "NotLoggedIn" }
}

impl fmt::Display for NotLoggedIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
