extern crate iron;
extern crate serde;
extern crate serde_json;

use iron::headers;
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status::Status;

pub fn json_response<T: serde::Serialize>(status: Status, body: T) -> IronResult<Response> {
    Ok(Response::with((
        status,
        Header(headers::ContentType::json()),
        &serde_json::to_string(&body).unwrap() as &str,
    )))
}
