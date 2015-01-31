extern crate toml;

use std::old_io::File;
use std::collections::BTreeMap;

use toml::Value;

pub struct ConfigReader
{
    config_map: BTreeMap<String, Value>,
}

impl ConfigReader
{
    // All methods will be here.
    pub fn new() -> ConfigReader
    {
        let configstring = File::open(&Path::new("config.toml")).read_to_string().unwrap();
        let value = toml::Parser::new(configstring.as_slice()).parse().unwrap();
        ConfigReader { config_map: value }
    }
}
