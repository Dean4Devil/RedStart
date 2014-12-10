extern crate toml;

use std::io::File;

pub struct ConfigReader
{
  // All attributes are declared here.
  file_content: toml::Value,
}

impl ConfigReader
{
  // All methods will be here.
  pub fn new() -> ConfigReader
  {
    let toml_code = String::from_utf8(File::open(&Path::new("config/redstart.toml")).read_to_end().unwrap()).unwrap();
    let value: toml::Value = from_str(toml_code.as_slice()).unwrap();
    ConfigReader { file_content: value }
  }
  pub fn get_string(&mut self, field_name: &str) -> Option<String>
  {
    let value = self.file_content.lookup(field_name).unwrap();
    let mut valid = false;
    match field_name
    {
      "Networking.address" => valid = true,
      "Logging.logfile" => valid = true,
      "General.name" => valid = true,
      _ => valid = false,
    }
    if valid == false
    {
      return None;
    }
    Some(value.as_str().unwrap().to_string())
  }
  pub fn get_integer(&mut self, field_name: &str) -> Option<i64>
  {
    let value = self.file_content.lookup(field_name).unwrap();
    let mut valid = false;
    match field_name
    {
      "Networking.port" => valid = true,
      _ => valid = false,
    }
    if valid == false
    {
      return None;
    }
    value.as_integer()
  }
  pub fn get_bool(&mut self, field_name: &str) -> Option<bool>
  {
    let value = self.file_content.lookup(field_name).unwrap();
    let mut valid = false;
    match field_name
    {
      _ => valid = false,
    }
    if valid == false
    {
      return None;
    }
    value.as_bool()
  }
}
