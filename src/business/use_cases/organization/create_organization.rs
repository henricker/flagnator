use std::error::Error;

use crate::business::repositories::organization_repository::OrganizationRepository;
use crate::{
    business::services::hash_service::HashService, domain::entities::organization::Organization,
};
use uuid::Uuid;

use super::errors::CreateOrganizationError;

pub struct CreateOrganizationUseCase<'a> {
    repository: &'a dyn OrganizationRepository,
    hash_service: &'a dyn HashService,
}

impl<'a> CreateOrganizationUseCase<'a> {
    pub fn new(
        repository: &'a dyn OrganizationRepository,
        hash_service: &'a dyn HashService,
    ) -> Self {
        CreateOrganizationUseCase {
            repository,
            hash_service,
        }
    }

    pub fn execute(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<Organization, Box<dyn Error>> {
        let email_exists = self.repository.email_exists(&email)?;

        if email_exists {
            return Err(Box::new(CreateOrganizationError::EmailExists));
        }

        let password_hash = self.hash_service.hash(&password);
        let id = Uuid::new_v4();
        let organization = Organization::new(id, name, email, password_hash);

        self.repository.add(&organization)?;

        Ok(organization)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::business::{
        repositories::organization_repository::MockOrganizationRepository,
        services::hash_service::MockHashService,
    };

    use super::*;

    #[test]
    fn test_create_organization_success() {
        // Arrange
        let mut mock_organization_repo = MockOrganizationRepository::new();
        let mut mock_hash_service = MockHashService::new();

        mock_organization_repo
            .expect_email_exists()
            .with(eq("test@example.com"))
            .times(1)
            .returning(|_| Ok(false));

        mock_organization_repo
            .expect_add()
            .times(1)
            .returning(|_| Ok(()));

        mock_hash_service
            .expect_hash()
            .with(eq("password123"))
            .times(1)
            .return_const("hashed_password".to_owned());

        let use_case = CreateOrganizationUseCase::new(&mock_organization_repo, &mock_hash_service);

        // Act
        let result = use_case.execute(
            "TestOrg".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
        );

        // Assert
        assert!(result.is_ok());
        let org = result.unwrap();
        assert_eq!(org.name, "TestOrg");
        assert_eq!(org.email, "test@example.com");
        assert_eq!(org.password, "hashed_password");
    }

    #[test]
    fn test_create_organization_email_exists() {
        // Arrange
        let mut mock_organization_repo = MockOrganizationRepository::new();
        let mock_hash_service = MockHashService::new();

        mock_organization_repo
            .expect_email_exists()
            .with(eq("existing@example.com"))
            .times(1)
            .returning(|_| Ok(true));

        let use_case = CreateOrganizationUseCase::new(&mock_organization_repo, &mock_hash_service);

        // Act
        let result = use_case.execute(
            "ExistingOrg".to_string(),
            "existing@example.com".to_string(),
            "password123".to_string(),
        );

        //Assert
        assert!(result.is_err());
        match *result
            .unwrap_err()
            .downcast::<CreateOrganizationError>()
            .unwrap()
        {
            CreateOrganizationError::EmailExists => (),
            _ => panic!("Expected EmailExists error"),
        }
    }
}
