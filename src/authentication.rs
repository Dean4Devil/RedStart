pub mod Authentication
{
    use std::error::Error;
    use std::fmt::{self, Debug};

    #[derive(Debug)]
    pub struct AuthError;

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
}
