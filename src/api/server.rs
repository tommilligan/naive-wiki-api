extern crate bodyparser;
extern crate iron;
extern crate logger;
extern crate persistent;

use iron::prelude::*;
use iron::Chain;
use router::Router;

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

pub fn start(address: &str) -> iron::Listening {
    let (logger_before, logger_after) = logger::Logger::new(None);
    let mut router = Router::new();
    router.get("/health", super::health, "HEALTH");

    let mut chain = Chain::new(router);
    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(
        MAX_BODY_LENGTH,
    ));
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    info!("Listening at {}", address);
    Iron::new(chain).http(address).unwrap()
}
