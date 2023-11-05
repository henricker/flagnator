use std::str::FromStr;

use domain::entities::organization::Organization;
use uuid::Uuid;

use crate::{
    business::repositories::organization_repository::OrganizationRepository,
    infrastructure::database::{
        database_connection::DbConnection,
        repositories::organization_diesel::OrganizationDieselRepository,
    },
};

mod business;
mod domain;
mod infrastructure;
mod shared;

fn main() {
    let db_connection = DbConnection {
        database_name: "flagnator".to_string(),
    };

    let repository = OrganizationDieselRepository {
        db_conn: db_connection,
    };

    //let _ = repository.add(&Organization::new(Uuid::new_v4(), "featrure-x".to_string(), "email@email.com".to_string(), "passwqord".to_string()));

    //let result_opt = repository.get(Uuid::from_str("1f0382aa-a35c-4d6c-8e8e-ca840a713c52").expect("error")).unwrap();

    let result = repository.email_exists("email@email.com").unwrap();

    println!("{}", result);
}
