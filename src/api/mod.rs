//! This module contains all the API sub-modules that are useable by RedStart
use configreader::ConfigReader;
use session::Store;

pub struct API
{
    pub config: ConfigReader,
    pub sessions: Store,
}

impl API
{
    pub fn new() -> API
    {
        API
        {
            config: ConfigReader::new(),
            sessions: Store::new(),
        }
    }
}
