use std::error::Error;

use diesel::prelude::Insertable;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::infrastructure::database::database_connection::DbConnection;
use crate::infrastructure::database::entities::organization_diesel_entity::OrganizationDiesel;
use uuid::Uuid;

use crate::infrastructure::database::error::DatabaseError;
use crate::infrastructure::database::schema::organizations::{self, email};
use crate::{
    business::repositories::organization_repository::OrganizationRepository,
    domain::entities::organization::Organization,
};
pub struct OrganizationDieselRepository {
    pub db_conn: DbConnection,
}

#[derive(Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::organizations)]
struct NewOrganization<'a> {
    pub id: &'a String,
    pub name: &'a String,
    pub password: &'a String,
    pub email: &'a String,
}

impl OrganizationRepository for OrganizationDieselRepository {
    fn add(&self, organization: &Organization) -> Result<(), Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();

        let new_organization = NewOrganization {
            email: &organization.email,
            id: &organization.id.to_string(),
            name: &organization.name,
            password: &organization.password,
        };

        diesel::insert_into(organizations::table)
            .values(&new_organization)
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
            Some(organization_schema) => Organization {
                email: organization_schema.email,
                id: Uuid::parse_str(&organization_schema.id.to_string())
                    .expect("Failed to load uuid"),
                name: organization_schema.name,
                password: organization_schema.password,
            },
        };

        Ok(Some(organization))
    }

    fn email_exists(&self, email_to_check: &str) -> Result<bool, Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();

        // Perform a query to check if any organization with the given email exists
        let exists = diesel::select(diesel::dsl::exists(
            organizations::dsl::organizations.filter(email.eq(email_to_check)),
        ))
        .get_result(conn)?;

        Ok(exists)
    }
}
