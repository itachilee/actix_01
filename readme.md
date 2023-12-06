# this is a project for learning actix lib

## features

- route

- log

- database

- json

## libraries

    [dependencies]
    actix-web = "4"
    serde_json = "1"
    futures = "0.3"
    serde = { version = "1.0", features = ["derive"] }
    actix-multipart = "0.6.0"
    actix-files = "0.6.0"
    derive_more = "0.99"
    env_logger = "0.10"
    utoipa-swagger-ui = { version = "5", features = ["actix-web"] }
    utoipa = "4.1"
    log = "0.4.0"
    chrono = "0.4"

## Auto-Reloading Development Server

During development it can be very handy to have cargo automatically recompile the code on changes. This can be accomplished very easily by using cargo-watch.

    cargo watch -x run

Historical Note
An old version of this page recommended using a combination of systemfd and listenfd, but this has many gotchas and was difficult to integrate properly, especially when part of a broader development workflow. We consider cargo-watch to be sufficient for auto-reloading purposes.

or use the following

Run server with auto-reloading:

    cargo install systemfd cargo-watch
    systemfd --no-pid -s http::8000 -- cargo watch -x run

---
