//! MySQL Driver implementation
use mysql::conn::MyOpts;

pub struct MySQL
{
    opts: MyOpts,
}

impl MySQL
{
    pub fn new() -> MySQL
    {
        MySQL
        {
            opts: MyOpts
            {
                user: Some("root".to_string()),
                pass: Some("DidRPwfMySQL".to_string()),
                db_name: Some("reservator".to_string()),
                tcp_port: 3306,
            },
        }
    }
}
