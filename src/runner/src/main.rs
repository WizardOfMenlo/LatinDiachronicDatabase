use clap::{load_yaml, App};
use graphql_queries::context::Context;
use graphql_queries::schema;
use query_driver::driver_init;
use query_driver::Configuration;
use query_driver::LemmMode;
use salsa::ParallelDatabase;
use std::sync::Arc;
use std::sync::Mutex;
use warp::{http::Response, Filter};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).get_matches();

    // If I fail, I want to see it :)
    color_backtrace::install();
    std::env::set_var("RUST_BACKTRACE", "1");

    // Init logging
    std::env::set_var("RUST_LOG", "warp_server");
    env_logger::init();

    // Initialize the db
    let db = Arc::new(Mutex::new(
        driver_init(
            Configuration::new(
                app.value_of("data_path").unwrap(),
                app.value_of("lemmatizer").unwrap(),
                if app.value_of("useLemlat").is_some() {
                    LemmMode::LemlatFormat
                } else {
                    LemmMode::CSVFormat
                },
            )
            .unwrap(),
        )
        .unwrap(),
    ));

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
    log::info!("Listening on 127.0.0.1:8080");

    // This is snapshot of the db
    let state = warp::any().map(move || Context::new(db.clone().lock().unwrap().snapshot()));

    // Create the graphql instance
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    // Serve all
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
