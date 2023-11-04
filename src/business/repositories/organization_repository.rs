use crate::domain::entities::organization::Organization;
use uuid::Uuid;

use std::error::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait OrganizationRepository {
    fn add(&self, organization: &Organization) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: Uuid) -> Result<Option<Organization>, Box<dyn Error>>;
    fn email_exists(&self, email: &str) -> Result<bool, Box<dyn Error>>;
}
