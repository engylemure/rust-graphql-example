#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod db;
mod errors;
mod graphql;
mod models;
mod utils;
mod web_utils;
mod schema;

use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpServer, web::Data};
use listenfd::ListenFd;

use crate::db::{mysql, redis};
use crate::graphql::{Mutation, QueryRoot, Schema, Subscription};
use crate::utils::env::ENV;
use crate::web_utils::handlers::{gql, gql_playground, gql_subscriptions};

fn create_schema() -> Schema {
    Schema::build(QueryRoot, Mutation, Subscription).finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = create_schema();
    let mut listenfd = ListenFd::from_env();
    let mysql_pool = Data::new(mysql::connect());
    let redis_conn = Data::new(redis::connect());
    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .app_data(mysql_pool.clone())
            .app_data(redis_conn.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/").guard(guard::Post()).to(gql))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(gql_subscriptions),
            )
            .service(web::resource("/").guard(guard::Get()).to(gql_playground))
    });

    server = if let Some(tcp_listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(tcp_listener).unwrap()
    } else {
        server.bind(format!("0.0.0.0:{}", ENV.server_port)).unwrap()
    };
    println!("Started http server: 0.0.0.0:{}", ENV.server_port);
    server.run().await
}
