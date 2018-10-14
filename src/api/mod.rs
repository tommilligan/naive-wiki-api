pub mod server;
pub mod utils;

use chrono::NaiveDateTime;
use iron::prelude::*;
use iron::status::Status;
use router::Router;

use self::utils::json_response;
use super::establish_connection;
use super::models;

const TIMESTAMP_FORMAT: &str = "%Y%m%dT%H%M%S%f";

pub fn health(req: &mut Request) -> IronResult<Response> {
    let conn = establish_connection();
    Ok(Response::with((Status::Ok)))
}
