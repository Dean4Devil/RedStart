use std::io::File;

pub struct ConfigReader
{
  // All attributes are declared here.
  file_content: toml::Value,
}

pub enum LogLevel
{
  SILENT,
  NORMAL,
  VERBOSE,
}

pub enum ConfigEntryType
{
  STRING,
  INTEGER,
  LOGLEVEL,
}

pub struct ConfigEntry
{
  type: ConfigEntryType,
  str_val: str,
  int_val: int,
  log_lvl_val: LogLevel,
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
  pub fn get_entry(entry: str) -> ConfigEntry
  {
    let value = file_content.lookup(entry).unwrap();

  }
}
