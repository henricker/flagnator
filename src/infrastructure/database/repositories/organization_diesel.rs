use std::error::Error;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::infrastructure::database::database_connection::DbConnection;
use crate::infrastructure::database::entities::organization_diesel_entity::OrganizationDiesel;
use crate::infrastructure::database::mappers::organization_db_mapper::OrganizationDbMapper;
use crate::shared::mappers::DbMapper;
use uuid::Uuid;

use crate::infrastructure::database::error::DatabaseError;
use crate::infrastructure::database::schema::organizations::{self, email};
use crate::{
    business::repositories::organization_repository::OrganizationRepository,
    domain::entities::organization::Organization,
};
pub struct OrganizationDieselRepository {
    pub db_conn: DbConnection,
    pub organization_db_mapper: OrganizationDbMapper,
}

impl OrganizationRepository for OrganizationDieselRepository {
    fn add(&self, organization: &Organization) -> Result<(), Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();

        let new_organization = self.organization_db_mapper.to_db(organization);

        diesel::insert_into(organizations::table)
            .values(new_organization)
            .returning(OrganizationDiesel::as_returning())
            .get_result(conn)
            .expect("Error saving new post");

        Ok(())
    }

    fn get(&self, id: Uuid) -> Result<Option<Organization>, Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();

        let result: Result<Option<OrganizationDiesel>, diesel::result::Error> =
            organizations::dsl::organizations
                .find(id.to_string())
                .select(OrganizationDiesel::as_select())
                .first(conn)
                .optional();

        let organization_opt = match result {
            Err(_) => return Err(Box::new(DatabaseError::GetOrganizationError)),
            Ok(organization_opt) => organization_opt,
        };

        let organization = match organization_opt {
            None => return Ok(None),
            Some(organization_schema) => {
                self.organization_db_mapper.to_entity(&organization_schema)
            }
        };

        Ok(Some(organization))
    }

    fn email_exists(&self, email_to_check: &str) -> Result<bool, Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();

        let exists = diesel::select(diesel::dsl::exists(
            organizations::dsl::organizations.filter(email.eq(email_to_check)),
        ))
        .get_result(conn)?;

        Ok(exists)
    }
}
