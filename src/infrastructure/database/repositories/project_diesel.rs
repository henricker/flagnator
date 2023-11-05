use std::error::Error;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::infrastructure::database::schema::projects::{self, name, organization_id};
use crate::{
    business::repositories::project_repository::ProjectRepository,
    domain::entities::project::Project,
    infrastructure::database::{
        database_connection::DbConnection, entities::project_diesel_entity::ProjectDiesel,
        error::DatabaseError, mappers::project_db_mapper::ProjectDbMapper,
    },
    shared::mappers::DbMapper,
};

pub struct ProjectDieselRepository {
    pub db_conn: DbConnection,
    pub project_db_mapper: ProjectDbMapper,
}

impl ProjectRepository for ProjectDieselRepository {
    fn add(&self, project: &Project) -> Result<(), Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();
        let new_project = self.project_db_mapper.to_db(project);

        diesel::insert_into(projects::table)
            .values(new_project)
            .returning(ProjectDiesel::as_returning())
            .get_result(conn)
            .expect("Error saving new project");

        Ok(())
    }

    fn get(&self, id: Uuid) -> Result<Option<Project>, Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();

        let result: Result<Option<ProjectDiesel>, diesel::result::Error> = projects::dsl::projects
            .find(id.to_string())
            .select(ProjectDiesel::as_select())
            .first(conn)
            .optional();

        let project_opt = match result {
            Err(_) => return Err(Box::new(DatabaseError::GetProjectError)),
            Ok(project_opt) => project_opt,
        };

        let project = match project_opt {
            None => return Ok(None),
            Some(project_schema) => self.project_db_mapper.to_entity(&project_schema),
        };

        Ok(Some(project))
    }

    fn name_exists(
        &self,
        name_to_check: &str,
        organization_id_to_check: Uuid,
    ) -> Result<bool, Box<dyn Error>> {
        let conn = &mut self.db_conn.get_database_connection();

        let exists = diesel::select(diesel::dsl::exists(
            projects::dsl::projects
                .filter(name.eq(name_to_check))
                .filter(organization_id.eq(organization_id_to_check.to_string())),
        ))
        .get_result(conn)?;

        Ok(exists)
    }
}
