
use uuid::Uuid;

use std::error::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::domain::entities::feature_flag::FeatureFlag;

#[derive(Debug)]
pub struct UpdateFeatureFlag {
    pub name: Option<String>,
    pub is_enabled: Option<bool>
}

#[cfg_attr(test, automock)]
pub trait FeatureFlagRepository {
    fn add(&self, feature_flag: &FeatureFlag) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: Uuid, project_id: Uuid) -> Result<Option<FeatureFlag>, Box<dyn Error>>;
    fn name_exists(&self, name: &str, project_id: Uuid) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, feature_flag_id: Uuid) -> Result<(), Box<dyn Error>>;
    fn update(&self, feature_flag_id: Uuid, update_feature_flag: UpdateFeatureFlag) -> Result<FeatureFlag, Box<dyn Error>>;
}