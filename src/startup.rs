use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(listener: TcpListener, pg_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let pg_connection_pool = web::Data::new(pg_connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pg_connection_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
