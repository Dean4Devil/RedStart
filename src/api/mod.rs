/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v. 2.0
 *
 * © Gregor Reitzenstein
 */

//! This module contains all the API sub-modules that are useable by RedStart

use configreader::ConfigReader;
use session::Store;
use data::MySQL;

pub use self::ggnet::GGNet;

mod ggnet;

#[derive(Clone)]
pub struct API
{
    pub config: ConfigReader,
    pub sessions: Store,
    pub ggnet: GGNet,
    pub mysql: MySQL,
}

impl API
{
    pub fn new() -> API
    {
        let mut config = ConfigReader::new();
        let mut mysql = MySQL::new(&mut config);
        let mut ggnet = GGNet::new(&mut config);
        API
        {
            config: config,
            sessions: Store::new(),
            ggnet: ggnet,
            mysql: mysql,
        }
    }
}
