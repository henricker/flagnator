use uuid::Uuid;

use super::schedule::Schedule;

#[derive(Debug)]
pub struct FeatureFlag {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub is_enabled: bool,
    pub schedule: Option<Schedule>,
}

impl FeatureFlag {
    // Constructor and other methods for the FeatureFlag entity
    pub fn new(id: Uuid, project_id: Uuid, name: String, is_enabled: bool, schedule: Option<Schedule>) -> FeatureFlag {
        FeatureFlag { 
            id, 
            project_id, 
            name, 
            is_enabled, 
            schedule 
        }
    }
}