//! Session management

use iron::typemap::Key;

pub struct Session
{
	pub key: String,
}

impl Key for Session { type Value = String; }

impl Session
{

}

pub mod Store
{
	mod PostgreSQL
	{
		// A PostgreSQL store
	}
	mod MySQL
	{
		// A MySQL store
	}
	mod FlatFile
	{
		// A flatfile store
	}
}
