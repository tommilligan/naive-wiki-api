use super::schema::{documents, revisions};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable)]
pub struct Document {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Document)]
pub struct Revision {
    pub id: i32,
    pub document_id: i32,
    pub created: NaiveDateTime,
    pub contents: String,
}

#[derive(Insertable)]
#[table_name = "documents"]
pub struct NewDocument<'a> {
    pub title: &'a str,
}

#[derive(Insertable)]
#[table_name = "revisions"]
pub struct NewRevision<'a> {
    pub document_id: i32,
    pub created: &'a NaiveDateTime,
    pub contents: &'a str,
}
