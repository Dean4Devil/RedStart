//! Around Middleware providing Authentication
//! This Middleware parses incoming requests for cookies and sets cookie headers on responses as necessary


pub mod Authentication
{
    use iron::{IronResult, IronError};
    use iron::status;

    use session::Session;

    use std::error::Error;
    use std::fmt::{self, Debug};

    #[derive(Debug)]
    pub struct AuthTimeout;
    #[derive(Debug)]
    pub struct AuthError;

    impl Error for AuthTimeout
    {
        fn description(&self) -> &'static str { "AuthTimeout" }
    }

    impl fmt::Display for AuthTimeout
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            Debug::fmt(self, f)
        }
    }

    impl Error for AuthError
    {
        fn description(&self) -> &'static str { "AuthError" }
    }

    impl fmt::Display for AuthError
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            Debug::fmt(self, f)
        }
    }

    pub fn parse_from_token(token: &str) -> IronResult<Session>
    {
        // TODO Remove Placeholders
        match token
        {
            // Placeholder correct token
            "UUXzTqbFRdzbr79" =>
            {
                return Ok(Session{ key: "testkey".to_string(), username: "testuser".to_string() })
            },
            // Placeholder outdated token
            "ixxKo5obDmees6o" =>
            {
                // TODO This should return a specific error.
                return Err(IronError::new(AuthTimeout, status::Unauthorized));
            }
            // Placeholder invalid token
            _ =>
            {
                return Err(IronError::new(AuthError, status::Unauthorized));
            }
        }
    }

    pub mod API
    {
    }

}
