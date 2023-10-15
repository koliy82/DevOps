
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use colored::Colorize;
use diesel::{PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::connection::establish_connection;
use crate::models::Airport;

pub mod connection;
pub mod schema;
pub mod models;
pub mod model_actions;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/airports")]
async fn get_airports(pool: web::Data<DbPool>) -> impl Responder{
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let response = Airport::get_all(&mut *conn);
    return match response {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let format = format!(
        "Status Code - {}{}{}{}{}{}",
        "%s ".green(),
        "IP - ", "%a ".cyan(),
        "time:", "%T ms".magenta(),
        "\nUser-Agent: %{User-Agent}i".bright_yellow());
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("{}", "Starting HTTP server...".yellow());
    let pool = establish_connection().expect("Cant create pool");
    log::info!("{}", "HTTP is started!".green());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_airports)
            .wrap(Logger::new(format.as_str()))
    })
        .bind(("0.0.0.0", 8989))
        .unwrap()
        .run()
        .await
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: u16,
    pub message: String,
}
