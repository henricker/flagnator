use uuid::Uuid;

use super::feature_flag::FeatureFlag;

#[derive(Debug)]
pub struct Project {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub feature_flags: Vec<FeatureFlag>,
}

impl Project {
    // Constructor and other methods for the Project entity
    pub fn new(id: Uuid, organization_id: Uuid, name: String, feature_flags: Vec<FeatureFlag>) -> Project {
        Project { 
            id, 
            organization_id, 
            name, 
            feature_flags 
        }
    }
}