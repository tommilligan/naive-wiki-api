extern crate chrono;
extern crate iron;
extern crate naive_wiki;
extern crate reqwest;

use std::collections::HashMap;

use chrono::offset::Utc;
use iron::Listening;
use naive_wiki::api::server::start;
use naive_wiki::api::RefRevision;
use reqwest::{Client, StatusCode};

struct TestSystem {
    address: String,
    client: Client,
    server: Listening,
}

impl TestSystem {
    fn start(port: usize) -> TestSystem {
        let address = format!("localhost:{}", port).to_string();
        TestSystem {
            client: Client::new(),
            server: start(&address),
            address,
        }
    }

    fn path(&self, path: &str) -> String {
        format!("http://{}{}", &self.address, path)
    }
}

impl Drop for TestSystem {
    fn drop(&mut self) {
        self.server.close().unwrap();
    }
}

#[test]
fn health() {
    let system = TestSystem::start(8081);
    let response = reqwest::get(&system.path("/health")).unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn document_put_bad_request() {
    let system = TestSystem::start(8082);

    // POST version
    let mut map = HashMap::new();
    map.insert("malformed_key", "malformed_value");
    let response = &system
        .client
        .post(&system.path("/documents/malformed_document"))
        .json(&map)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn document_put_and_get() {
    let system = TestSystem::start(8083);

    // POST version
    let mut map = HashMap::new();
    map.insert("content", "test_document contents");
    let mut response = system
        .client
        .post(&system.path("/documents/test_document"))
        .json(&map)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // GET content back
    let revision: RefRevision = response.json().unwrap();
    let check = reqwest::get(&system.path(&revision.url)).unwrap();
}

#[test]
fn document_put_and_get_le() {
    let system = TestSystem::start(8084);

    // POST version
    let mut map = HashMap::new();
    map.insert("content", "test_document contents");
    let mut response = system
        .client
        .post(&system.path("/documents/test_document_le"))
        .json(&map)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // GET content back
    let revision: RefRevision = response.json().unwrap();
    let now = Utc::now().naive_utc();
    let check =
        reqwest::get(&system.path(&format!("/documents/test_docment_le/{}", &now))).unwrap();
}
