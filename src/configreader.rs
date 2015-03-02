use std::os;
use std::old_io;
use std::old_io::{fs, File, IoErrorKind};
use std::collections::BTreeMap;
use std::sync::mpsc::{Sender, Receiver};

use toml::{self, Value};

use serialize::Decodable;

pub struct ConfigReader
{
    config_map: Value, //BTreeMap<String, Value>,
}

impl ConfigReader
{
    pub fn new() -> ConfigReader
    {
        let curr_dir = os::self_exe_path().expect("huh?");
        // Create the configuration directory if it does not exist yet.
        let configdir = curr_dir.join("config/");
        //let configdir = Path::new("config/");
        // This returns a Result with an error if the directory already exists or the user does not
        // have write permissions. We ignore that possibility for now.
        fs::mkdir(&configdir, old_io::USER_RWX);

        // Open the configuration file.
        let configpath  = curr_dir.join("config/redstart.toml");
        //let configpath  = Path::new("config/redstart.toml");
        let mut configfile = match File::open(&configpath)
        {
            // File does exists -> return file descriptor.
            Ok(f) => f,
            // File does *not* exist -> create file, dump default config, return file descriptor
            Err(e) =>
            {
                match e.kind
                {
                    IoErrorKind::PermissionDenied => panic!("Could not read config file. Does it have the correct permissions?"),
                    IoErrorKind::FileNotFound =>
                    {
                        let mut fd = File::create(&configpath);
                        fd.write(b"[General]\r\nname=\"RedStart\"\r\n[Networking]\r\naddress = \"127.0.0.1\"\r\nport = 8080\r\n[Logging]\r\nloglevel = \"NORMAL\"\r\nlogfile = \"log/default.log\"\r\n"
);
                        let mut fd = File::open(&configpath);
                        fd.unwrap()
                    }
                    _ => panic!("File error: {}!", e),
                }
            }
        };
        let configstring = configfile.read_to_string().unwrap();
        let mut configparser = toml::Parser::new(configstring.as_slice());
        let table = match configparser.parse()
        {
            Some(val) => val,
            None => panic!("Configfile parse error! Check config syntax!"),
        };
        println!("{:?}", table);
        let value: Value = Value::Table(table);
        ConfigReader { config_map: value }
    }

    /// Return the value that belongs to the key given. If no value was found, `None` will be returned
    pub fn get_value<T: Decodable>(&mut self, key: &str) -> Option<T>
    {
        match self.config_map.lookup(key)
        {
            Some(value) => toml::decode::<T>(value.clone()),
            None => None
        }
    }

    /// Return the value that belongs to the key given. If no value was found, a given default value
    /// will be returned.
    pub fn get_value_or<T: Decodable>(&mut self, key: &str, default: T) -> T
    {
        match self.get_value::<T>(key)
        {
            Some(e) => e,
            None => default,
        }
    }
}
