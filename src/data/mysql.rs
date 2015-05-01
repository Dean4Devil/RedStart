//! MySQL Driver implementation
use mysql::conn::{MyOpts, pool};

#[derive(Clone)]
pub struct MySQL
{
    pub pool: pool::MyPool,
}

fn get_opts() -> MyOpts 
{
    MyOpts
    {
        user: Some("root".to_string()),
        pass: Some("DidRPwfMySQL".to_string()),
        db_name: Some("reservator".to_string()),
        tcp_port: 3306,
        tcp_addr: Some("127.0.0.1".to_string()),
        unix_addr: None,
        prefer_socket: true,
        init: vec![],
        verify_peer: false,
        ssl_opts: None,
    }
}

impl MySQL
{
    pub fn new() -> MySQL
    {
        let opts = get_opts();
        MySQL
        {
            pool: pool::MyPool::new(opts).unwrap(),
        }
    }
}
