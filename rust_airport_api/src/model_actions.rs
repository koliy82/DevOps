use std::error::Error;
use diesel::{PgConnection, RunQueryDsl};
use crate::models::Airport;

impl Airport{
    pub fn get_all(connection: &mut PgConnection) -> Result<Vec<Airport>, Box<dyn Error + Send + Sync>>{
        use crate::schema::{airport};
        let response = airport::dsl::airport
            .load::<Airport>(connection)
            .expect("Error when get categories");
        Ok(response)
    }
}