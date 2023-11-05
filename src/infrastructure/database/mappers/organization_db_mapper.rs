use uuid::Uuid;

use crate::domain::entities::organization::Organization;
use crate::infrastructure::database::entities::organization_diesel_entity::OrganizationDiesel;
use crate::shared::mappers::DbMapper;

pub struct OrganizationDbMapper {}

impl DbMapper<Organization, OrganizationDiesel> for OrganizationDbMapper {
    fn to_db(entity: Organization) -> OrganizationDiesel {
        OrganizationDiesel {
            email: entity.email,
            id: entity.id.to_string(),
            name: entity.name,
            password: entity.password,
        }
    }

    fn to_entity(model: OrganizationDiesel) -> Organization {
        Organization {
            email: model.email,
            id: Uuid::parse_str(&model.id.to_string()).expect("Failed to load uuid"),
            name: model.name,
            password: model.password,
        }
    }
}
