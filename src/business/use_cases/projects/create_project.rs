use std::error::Error;

use crate::business::repositories::project_repository::ProjectRepository;
use crate::domain::entities::project::Project;

use uuid::Uuid;

use super::errors::CreateProjectError;

pub struct CreateProjectUseCase<'a> {
    repository: &'a dyn ProjectRepository,
}

impl<'a> CreateProjectUseCase<'a> {
    pub fn new(repository: &'a dyn ProjectRepository) -> Self {
        CreateProjectUseCase { repository }
    }

    pub fn execute(&self, name: String, organization_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let name_exists = self.repository.name_exists(&name, organization_id)?;

        if name_exists {
            return Err(Box::new(CreateProjectError::NameExists));
        }

        let id = Uuid::new_v4();
        let project = Project::new(id, organization_id, name, None);

        self.repository.add(&project)?;

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use crate::business::repositories::project_repository::MockProjectRepository;
    use mockall::predicate::eq;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_create_project_name_exists() {
        // Arrange
        let mut mock_project_repo = MockProjectRepository::new();

        let uuid = Uuid::new_v4();

        mock_project_repo
            .expect_name_exists()
            .with(eq("name_existent"), eq(uuid))
            .times(1)
            .returning(|_, _| Ok(true));

        let use_case = CreateProjectUseCase::new(&mock_project_repo);

        //Act
        let result = use_case.execute("name_existent".to_string(), uuid);

        //Assert
        assert!(result.is_err());
        match *result
            .unwrap_err()
            .downcast::<CreateProjectError>()
            .unwrap()
        {
            CreateProjectError::NameExists => (),
            _ => panic!("Expected NameExists error"),
        }
    }

    #[test]
    fn test_create_project_success() {
        // Arrange
        let mut mock_project_repo = MockProjectRepository::new();

        let uuid = Uuid::new_v4();

        mock_project_repo
            .expect_name_exists()
            .with(eq("name"), eq(uuid))
            .times(1)
            .returning(|_, _| Ok(false));

        mock_project_repo
            .expect_add()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = CreateProjectUseCase::new(&mock_project_repo);

        //Act
        let result = use_case.execute("name".to_string(), uuid);

        //Assert
        assert!(result.is_ok());
        let proj = result.unwrap();
        assert_eq!(proj.name, "name");
        assert_eq!(proj.organization_id, uuid);
    }
}
