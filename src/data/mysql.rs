//! MySQL Driver implementation
use std::path::PathBuf;

use mysql::conn::{MyOpts, pool};

use configreader::ConfigReader;

#[derive(Clone)]
pub struct MySQL
{
    pub pool: pool::MyPool,
}

fn get_opts(config: &mut ConfigReader) -> MyOpts 
{
    MyOpts
    {
        user: Some(config.get_value_or::<String>("MySQL.username", "root".to_string())),
        pass: Some(config.get_value_or::<String>("MySQL.password", "DidRPwfMySQL".to_string())),
        db_name: Some("reservator".to_string()),
        tcp_addr: Some(config.get_value_or::<String>("MySQL.address", "127.0.0.1".to_string())),
        tcp_port: config.get_value_or::<u16>("MySQL.port", 3306),
        unix_addr: Some(PathBuf::from(config.get_value_or::<String>("MySQL.socket", "/var/run/mysqld/mysqld.sock".to_string()))),
        prefer_socket: true,
        init: vec![],
        verify_peer: false,
        ssl_opts: None,
    }
}

impl MySQL
{
    pub fn new(config: &mut ConfigReader) -> MySQL
    {
        let opts = get_opts(config);
        MySQL
        {
            pool: pool::MyPool::new(opts).unwrap(),
        }
    }
}
