use uuid::Uuid;

use crate::domain::entities::organization::Organization;
use crate::infrastructure::database::entities::organization_diesel_entity::OrganizationDiesel;
use crate::shared::mappers::DbMapper;

pub struct OrganizationDbMapper {}

impl DbMapper<Organization, OrganizationDiesel> for OrganizationDbMapper {
    fn to_db(&self, entity: &Organization) -> OrganizationDiesel {
        OrganizationDiesel {
            email: entity.email.clone(),
            id: entity.id.to_string(),
            name: entity.name.clone(),
            password: entity.password.clone(),
        }
    }

    fn to_entity(&self, model: &OrganizationDiesel) -> Organization {
        Organization {
            email: model.email.clone(),
            id: Uuid::parse_str(&model.id.to_string()).expect("Failed to load uuid"),
            name: model.name.clone(),
            password: model.password.clone(),
        }
    }
}
