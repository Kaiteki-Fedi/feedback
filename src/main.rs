extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate lazy_static;
extern crate config;

use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};

pub mod app;
pub mod schema;
pub mod utils;

#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let listen_address: String = app::config::get("listen_address");
    let db_pool = Data::new(crate::app::db::get_connection_pool());

    println!("Listening to requests at {}...", listen_address);
    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(app::init::initialize)
            .wrap(middleware::Logger::default())
    })
    .bind(listen_address)?
    .run()
    .await
}
