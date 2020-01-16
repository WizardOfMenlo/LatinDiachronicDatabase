use salsa::ParallelDatabase;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use warp::{http::Response, Filter};

use latin_db::arguments::load_configuration;
use latin_db::graphql_queries::context::Context;
use latin_db::graphql_queries::schema;
use latin_db::query_driver::driver_init;
use latin_db::query_system::gc::GCollectable;

fn main() {
    // If I fail, I want to see it :)
    color_backtrace::install();
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "info,warp_server");
    env_logger::init();

    // Initialize the db
    let db = Arc::new(Mutex::new(driver_init(load_configuration()).unwrap()));
    let garbage_copy = db.clone();

    // Spawn a GC daemon
    thread::spawn(move || loop {
        thread::sleep(Duration::new(10, 0));
        let mut db = garbage_copy.lock().unwrap();
        db.garbage_sweep();
    });

    let log = warp::log("warp_server");

    // Redirect to graphiql
    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
                    .to_string(),
            )
    });

    // log that we are running!
    log::info!("Listening on 0.0.0.0.8088");

    // This is snapshot of the db
    let state = warp::any().map(move || Context::new(db.clone().lock().unwrap().snapshot()));

    // Set up cors. TODO, this is most likely insecure
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])
        .allow_headers(vec!["origin", "content-type", "accept"]);

    // Create the graphql instance
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    // Serve all
    warp::serve(
        warp::get2()
            .and(warp::path("healthz"))
            .map(|| warp::http::StatusCode::OK)
            .or(warp::path("graphiql")
                .and(juniper_warp::graphiql_filter("/graphql"))
                .or(homepage)
                .or(warp::path("graphql").and(graphql_filter)))
            .with(log)
            .with(cors),
    )
    .run(([0, 0, 0, 0], 8088));
}
