extern crate pretty_env_logger;

extern crate naive_wiki;

fn main() {
    pretty_env_logger::init();

    let address = "localhost:8080";
    naive_wiki::api::server::start(address);
}
