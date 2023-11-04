
use uuid::Uuid;

use std::error::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::domain::entities::project::Project;

#[cfg_attr(test, automock)]
pub trait ProjectRepository {
    fn add(&self, project: &Project) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: Uuid) -> Result<Option<Project>, Box<dyn Error>>;
    fn name_exists(&self, name: &str, organization_id: Uuid) -> Result<bool, Box<dyn Error>>;
}