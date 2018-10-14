# naive_wiki

## Run locally

To run the API, set up the database and environment as follows:

Environment setup:

```
docker-compose up -d db
docker-compose up db-migrate
cargo build
```

Run API:

```
DATABASE_URL=postgres://user_nwa:password_nwa@localhost:5432/db_nwa cargo run --bin naive_wiki_server
```

Run integration tests:

```
DATABASE_URL=postgres://user_nwa:password_nwa@localhost:5432/db_nwa cargo test
```
