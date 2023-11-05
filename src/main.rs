use std::str::FromStr;

use business::repositories::project_repository::ProjectRepository;
use domain::entities::project::Project;
use infrastructure::database::{
    mappers::project_db_mapper::ProjectDbMapper,
    repositories::project_diesel::ProjectDieselRepository,
};
use uuid::Uuid;

use crate::infrastructure::database::database_connection::DbConnection;

mod business;
mod domain;
mod infrastructure;
mod shared;

fn main() {
    let db_connection = DbConnection {
        database_name: "flagnator".to_string(),
    };

    let repository = ProjectDieselRepository {
        db_conn: db_connection,
        project_db_mapper: ProjectDbMapper {},
    };

    let _ = repository.add(&Project {
        feature_flags: None,
        id: Uuid::new_v4(),
        name: "project_a".to_string(),
        organization_id: Uuid::from_str("1f0382aa-a35c-4d6c-8e8e-ca840a713c52").unwrap(),
    });

    //let result_opt = repository.get(Uuid::from_str("7aa7861b-d842-4680-9c0d-2e9d1a756222").unwrap()).unwrap();

    // let result = repository.name_exists("project_b", Uuid::from_str("0f0382aa-a35c-4d6c-8e8e-ca840a713c52").unwrap()).unwrap();

    // println!("{:?}", result);
}
