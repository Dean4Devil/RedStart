use std::os;
use std::old_io;
use std::old_io::{fs, File, IoErrorKind};
use std::collections::BTreeMap;
use std::sync::mpsc::{Sender, Receiver};

use toml::{self, Value};

use serialize::Decodable;

#[derive(Clone)]
/// The configparser reads the config file and provides functions to access config values.
pub struct ConfigReader
{
    config_map: Value, //BTreeMap<String, Value>,
}

impl ConfigReader
{
    /// Create a new ConfigReader object. This should only be called in the setup() function.
    pub fn new() -> ConfigReader
    {
        // Get the current pwd
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
                        // There is no config file. So create one.
                        // This opens the file in writeable mode.
                        let mut fd = File::create(&configpath);
                        
                        // Write default values.
                        fd.write(b"[General]\nname=\"RedStart\"\n\n[Networking]\naddress = \"127.0.0.1\"\nport = 8080\n\n[Logging]\nloglevel = \"NORMAL\"\nlogfile = \"log/default.log\"\n\n[Security]\nhttps = false\ncertificate = \"../../ssl/cert.pem\"\nkey = \"../../ssl/key.pem\"\n"
);
                        // Open the file in readonly mode again
                        let mut fd = File::open(&configpath);
                        fd.unwrap()
                    }
                    _ => panic!("File error: {}!", e),
                }
            }
        };

        // Read the config-file into memory.
        let configstring = configfile.read_to_string().unwrap();

        // Create a new TOML Parser from the config
        let mut configparser = toml::Parser::new(configstring.as_slice());

        // Parse the TOML configfile into a Binary Tree map.
        let table = match configparser.parse()
        {
            Some(val) => val,
            None => panic!("Configfile parse error! Check config syntax!"),
        };
        println!("{:?}", table);

        // Save the BTreeMap into the struct
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
