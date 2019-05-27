use graphql_queries::context::Context;
use graphql_queries::schema;
use query_driver::driver_init;
use salsa::ParallelDatabase;
use std::sync::Arc;
use std::sync::Mutex;
use warp::{http::Response, Filter};

fn main() {
    color_backtrace::install();

    std::env::set_var("RUST_LOG", "warp_server");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db = Arc::new(Mutex::new(
        driver_init("./data/works/", "./data/out.txt").unwrap(),
    ));

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
                    .to_string(),
            )
    });

    log::info!("Listening on 127.0.0.1:8080");

    let state = warp::any().map(move || Context::new(db.clone().lock().unwrap().snapshot()));
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080));
}
