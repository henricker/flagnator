
use uuid::Uuid;

use std::error::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::domain::entities::feature_flag::FeatureFlag;


#[cfg_attr(test, automock)]
pub trait FeatureFlagRepository {
    fn add(&self, feature_flag: &FeatureFlag) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: Uuid, project_id: Uuid) -> Result<Option<FeatureFlag>, Box<dyn Error>>;
    fn name_exists(&self, name: &str, project_id: Uuid) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, feature_flag_id: Uuid) -> Result<(), Box<dyn Error>>;
}