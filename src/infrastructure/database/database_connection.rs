use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use std::env;

pub struct DbConnection {
    pub database_name: String
}

impl DbConnection {
    pub fn get_database_connection(&self) -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

