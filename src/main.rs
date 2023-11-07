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

fn main() {}
