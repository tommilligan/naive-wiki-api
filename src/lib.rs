pub mod api;
pub mod models;
pub mod schema;

extern crate bodyparser;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate persistent;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::offset::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

use self::schema::documents::dsl::*;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
