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

/// Get a document by its title
pub fn get_document<'a>(
    conn: &PgConnection,
    document_title: &'a str,
) -> QueryResult<models::Document> {
    documents
        .filter(schema::documents::dsl::title.eq(document_title))
        .get_result(conn)
}

/// Get a revision of a Document by timestamp
pub fn get_revision(
    conn: &PgConnection,
    document: &models::Document,
    revision_timestamp: Option<chrono::NaiveDateTime>,
) -> QueryResult<models::Revision> {
    match revision_timestamp {
        Some(ts) => models::Revision::belonging_to(document)
            .filter(schema::revisions::dsl::created.le(ts))
            .order_by(schema::revisions::dsl::created.desc())
            .first(conn),
        None => models::Revision::belonging_to(document)
            .order_by(schema::revisions::dsl::created.desc())
            .first(conn),
    }
}

/// List revisions of a Document
pub fn list_revisions(
    conn: &PgConnection,
    document: &models::Document,
) -> QueryResult<Vec<models::Revision>> {
    models::Revision::belonging_to(document).load::<models::Revision>(conn)
}

/// Create a new version of a Document
pub fn create_document<'a>(
    conn: &PgConnection,
    document_title: &'a str,
    contents: &'a str,
) -> QueryResult<(models::Document, models::Revision)> {
    // If the document exists, get it, otherwise inser it
    let existing_doc = get_document(conn, document_title);
    let doc: models::Document = match existing_doc {
        Ok(doc) => doc,
        Err(_) => {
            let new_doc = models::NewDocument {
                title: document_title,
            };
            diesel::insert_into(schema::documents::table)
                .values(&new_doc)
                .get_result(conn)?
        }
    };

    // New revision
    let now = Utc::now().naive_utc();
    let new_revision = models::NewRevision {
        document_id: doc.id,
        created: &now,
        contents,
    };
    let rev = diesel::insert_into(schema::revisions::table)
        .values(&new_revision)
        .get_result(conn)?;

    // Return new data objects
    Ok((doc, rev))
}

pub fn list_documents(conn: &PgConnection) -> Vec<models::Document> {
    let results = documents
        .load::<models::Document>(conn)
        .expect("Error loading documents");
    results
}
