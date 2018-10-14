extern crate iron;
extern crate naive_wiki;
extern crate reqwest;

use iron::Listening;
use naive_wiki::api::server::start;
use reqwest::{Client, StatusCode};

struct TestSystem {
    address: String,
    client: Client,
    server: Listening,
}

impl TestSystem {
    fn start() -> TestSystem {
        let address = "localhost:8089".to_string();
        TestSystem {
            client: Client::new(),
            server: start(&address),
            address,
        }
    }

    fn path(self, path: &str) -> String {
        format!("http://{}{}", &self.address, path)
    }
}

impl Drop for TestSystem {
    fn drop(&mut self) {
        self.server.close().unwrap();
    }
}

#[test]
fn documents() {
    let system = TestSystem::start();
    let response = reqwest::get(&system.path("/health")).unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
