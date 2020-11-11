// use actix::prelude::*;
use actix_web::{web, get, middleware, App, Error, HttpRequest, HttpResponse, HttpServer};
use clap::Clap;
// use simple_logger::SimpleLogger;

// https://www.poxnora.com/api/feed.do?t=json

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const NAME: &'static str = "PoxBase server";
const ABOUT: &'static str = "This is the server for the PoxBase service";

#[derive(Clap)]
#[clap(name = NAME, version = VERSION, author = AUTHORS, about = ABOUT)]
struct Opts {
    #[clap(
        short = 'l',
        long = "listen",
        default_value = "127.0.0.1:8000",
        about = "This is the socket the server is listening on. This is restricted localhost (127.0.0.1) by default and should be fine for most use cases. In a container, you likely want to set this to '0.0.0.0:8000'"
    )]
    socket: std::net::SocketAddr,
}

#[get("/hello/")]
async fn hello() -> Result<HttpResponse, Error> {
    HttpResponse::Ok().body("Hello").await
}

/// Telemetry entry point. Listening by default on 127.0.0.1:8000.
/// This can be changed using the `PORT` and `BIND` ENV variables.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // SimpleLogger::new().with_level(log::LevelFilter::Info).init().expect("Must be able to start a logger");

    let opts: Opts = Opts::parse();
    // let aggregator = Aggregator::new().start();
    // let factory = LocatorFactory::new();
    // let locator = SyncArbiter::start(4, move || factory.create());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(hello)
            // .data(aggregator.clone())
            // .data(locator.clone())
            // .service(node_route)
            // .service(feed_route)
            // .service(state_route)
            // .service(health)
    })
    .bind(opts.socket)?
    .run()
    .await
}
