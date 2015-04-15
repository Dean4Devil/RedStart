/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

//! This module contains all the API sub-modules that are useable by RedStart

use configreader::ConfigReader;
use session::Store;

pub use self::ggnet::GGNet;

mod ggnet;

#[derive(Clone)]
pub struct API
{
    pub config: ConfigReader,
    pub sessions: Store,
    pub ggnet: GGNet,
}

impl API
{
    pub fn new() -> API
    {
        API
        {
            config: ConfigReader::new(),
            sessions: Store::new(),
            ggnet: GGNet::new(),
        }
    }
}
