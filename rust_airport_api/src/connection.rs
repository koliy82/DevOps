use diesel::pg::PgConnection;
use std::env;
use std::error::Error;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub fn establish_connection() -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)
        .expect("Failed to create pool");
    Ok(pool)
}