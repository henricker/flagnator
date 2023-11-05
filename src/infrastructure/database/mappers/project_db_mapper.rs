use crate::domain::entities::project::Project;
use crate::infrastructure::database::entities::project_diesel_entity::ProjectDiesel;
use crate::shared::mappers::DbMapper;
use uuid::Uuid;

pub struct ProjectDbMapper {}

impl DbMapper<Project, ProjectDiesel> for ProjectDbMapper {
    fn to_db(&self, entity: &Project) -> ProjectDiesel {
        ProjectDiesel {
            id: entity.id.to_string().clone(),
            name: entity.name.clone(),
            organization_id: entity.organization_id.to_string().clone(),
        }
    }

    fn to_entity(&self, model: &ProjectDiesel) -> Project {
        Project {
            id: Uuid::parse_str(&model.id.to_string()).expect("Failed to load uuid"),
            name: model.name.clone(),
            organization_id: Uuid::parse_str(&model.organization_id.to_string())
                .expect("Failed to load uuid"),
            feature_flags: None,
        }
    }
}
