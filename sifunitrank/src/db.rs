use std::ops::Deref;

use super::r2d2;
use super::rusqlite;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

pub fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = SqliteConnectionManager::new_with_flags("unit.db_", rusqlite::SQLITE_OPEN_READ_ONLY);
    r2d2::Pool::new(config, manager).expect("db pool")
}

pub struct Conn(pub r2d2::PooledConnection<SqliteConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &Connection.
impl Deref for Conn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
