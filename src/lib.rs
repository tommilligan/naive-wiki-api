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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
