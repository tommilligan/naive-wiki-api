pub mod server;
pub mod utils;

extern crate bodyparser;

use chrono::NaiveDateTime;
use iron::prelude::*;
use iron::status::Status;
use router::Router;

use self::utils::json_response;
use super::establish_connection;
use super::models;

const TIMESTAMP_FORMAT: &str = "%Y%m%dT%H%M%S%f";

pub fn format_ts(ts: &NaiveDateTime) -> String {
    ts.format(TIMESTAMP_FORMAT).to_string()
}

// API request/response handlers

pub fn health(_req: &mut Request) -> IronResult<Response> {
    // Check DB connection
    let _conn = establish_connection();
    Ok(Response::with(Status::Ok))
}

// Generic error response
#[derive(Deserialize, Serialize)]
pub struct Error<'a> {
    pub error: bool,
    pub message: &'a str,
}

impl<'a> Error<'a> {
    pub fn new(message: &'a str) -> Error {
        Error {
            error: true,
            message,
        }
    }
}

// Reference document
#[derive(Deserialize, Serialize)]
pub struct RefDocument {
    pub title: String,
    pub url: String,
}

impl RefDocument {
    pub fn new(doc: &models::Document) -> RefDocument {
        RefDocument {
            url: format!("/documents/{}", &doc.title),
            title: doc.title.clone(),
        }
    }
}

// Reference revision
#[derive(Deserialize, Serialize)]
pub struct RefRevision {
    pub timestamp: String,
    pub url: String,
}

impl RefRevision {
    pub fn new(doc: &models::Document, rev: &models::Revision) -> RefRevision {
        let ts = format_ts(&rev.created);
        RefRevision {
            url: format!("/documents/{}/{}", &doc.title, &ts),
            timestamp: ts.to_string(),
        }
    }
}

// Content of document
#[derive(Deserialize, Serialize, Clone)]
pub struct Content {
    pub content: String,
}

// Get document list, map to reference
pub fn list_documents(_req: &mut Request) -> IronResult<Response> {
    let conn = establish_connection();
    let titles: Vec<RefDocument> = super::list_documents(&conn)
        .into_iter()
        .map(|doc| RefDocument::new(&doc))
        .collect();
    json_response(Status::Ok, titles)
}

// Get revision list, map to reference
pub fn list_revisions(req: &mut Request) -> IronResult<Response> {
    let conn = establish_connection();
    let title = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("title")
        .expect("Require document title");
    let doc = super::get_document(&conn, title).expect("Error getting documnet");
    let timestamps: Vec<RefRevision> = super::list_revisions(&conn, &doc)
        .expect("Error getting revisions")
        .into_iter()
        .map(|rev| RefRevision::new(&doc, &rev))
        .collect();
    json_response(Status::Ok, timestamps)
}

// Create revision, respond with error appropriately
pub fn create_revision(req: &mut Request) -> IronResult<Response> {
    let conn = establish_connection();
    let body = req.get::<bodyparser::Struct<Content>>();
    let ref title = req
        .extensions
        .get::<Router>()
        .unwrap()
        .find("title")
        .expect("Require document title");
    match body {
        Ok(Some(body)) => {
            let res = super::create_document(&conn, title, &body.content);
            match res {
                Ok((doc, rev)) => json_response(Status::Created, RefRevision::new(&doc, &rev)),
                Err(_) => json_response(
                    Status::InternalServerError,
                    Error::new("Failed to process update"),
                ),
            }
        }
        Ok(None) => json_response(Status::BadRequest, Error::new("No document body given")),
        Err(err) => {
            warn!("Error parsing document body; {}", err);
            json_response(Status::BadRequest, Error::new("Invalid document body"))
        }
    }
}

pub fn get_revision(req: &mut Request) -> IronResult<Response> {
    let conn = establish_connection();
    let router = req.extensions.get::<Router>().unwrap();
    let title = router.find("title").expect("Require title");
    let ts = router.find("revision").expect("Require revision");

    let doc = super::get_document(&conn, title).expect("Error getting document");
    let rev = if ts == "latest" {
        super::get_revision(&conn, &doc, None)
    } else {
        let dt =
            NaiveDateTime::parse_from_str(ts, TIMESTAMP_FORMAT).expect("Invalid timestamp given");
        super::get_revision(&conn, &doc, Some(dt))
    };

    match rev {
        Ok(rev) => json_response(
            Status::Ok,
            Content {
                content: rev.contents,
            },
        ),
        Err(_) => json_response(
            Status::InternalServerError,
            Error::new("Failed to fetch content"),
        ),
    }
}
