use std::error::Error;

use uuid::Uuid;

use crate::{
    business::repositories::{
        feature_flag_repository::FeatureFlagRepository, project_repository::ProjectRepository,
    },
    domain::entities::feature_flag::FeatureFlag,
};

use super::errors::CreateFeatureFlagError;

pub struct CreateFeatureFlagUseCase<'a> {
    project_repository: &'a dyn ProjectRepository,
    feature_flag_repository: &'a dyn FeatureFlagRepository,
}

impl<'a> CreateFeatureFlagUseCase<'a> {
    pub fn new(
        project_repository: &'a dyn ProjectRepository,
        feature_flag_repository: &'a dyn FeatureFlagRepository,
    ) -> Self {
        CreateFeatureFlagUseCase {
            project_repository,
            feature_flag_repository,
        }
    }

    pub fn execute(
        &self,
        name: String,
        project_id: Uuid,
        organization_id: Uuid,
    ) -> Result<FeatureFlag, Box<dyn Error>> {
        let project_opt = self.project_repository.get(project_id)?;

        let project = match project_opt {
            None => return Err(Box::new(CreateFeatureFlagError::ProjectNotFound)),
            Some(project) => project,
        };

        if project.organization_id != organization_id {
            return Err(Box::new(CreateFeatureFlagError::ProjectNotFound));
        }

        let feature_name_exists = self
            .feature_flag_repository
            .name_exists(&name, project_id)?;

        if feature_name_exists {
            return Err(Box::new(CreateFeatureFlagError::NameExists));
        }

        let new_feature_flag = FeatureFlag::new(Uuid::new_v4(), project_id, name, false, None);
        self.feature_flag_repository.add(&new_feature_flag)?;

        Ok(new_feature_flag)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        business::repositories::{
            feature_flag_repository::MockFeatureFlagRepository,
            project_repository::MockProjectRepository,
        },
        domain::entities::project::Project,
    };
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test_create_project_if_project_not_exists() {
        // Arrange
        let mut mock_project_repo = MockProjectRepository::new();
        let mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let organization_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();

        mock_project_repo
            .expect_get()
            .with(eq(project_uuid))
            .times(1)
            .returning(|_| Ok(None));

        let use_case = CreateFeatureFlagUseCase::new(&mock_project_repo, &mock_feature_flag_repo);

        // Act
        let result = use_case.execute("feature-x".to_string(), project_uuid, organization_uuid);

        // Assert
        assert!(result.is_err());
        match *result
            .unwrap_err()
            .downcast::<CreateFeatureFlagError>()
            .unwrap()
        {
            CreateFeatureFlagError::ProjectNotFound => (),
            _ => panic!("Expected ProjectNotFound error"),
        }
    }

    #[test]
    fn test_create_project_if_project_not_belongs_organization() {
        // Arrange
        let mut mock_project_repo = MockProjectRepository::new();
        let mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let organization_uuid = Uuid::new_v4();
        let other_organization_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();

        mock_project_repo
            .expect_get()
            .with(eq(project_uuid))
            .times(1)
            .returning(move |_| {
                Ok(Some(Project::new(
                    project_uuid,
                    other_organization_uuid,
                    "feature-x".to_string(),
                    None,
                )))
            });

        let use_case = CreateFeatureFlagUseCase::new(&mock_project_repo, &mock_feature_flag_repo);

        // Act
        let result = use_case.execute("feature-x".to_string(), project_uuid, organization_uuid);

        // Assert
        assert!(result.is_err());
        match *result
            .unwrap_err()
            .downcast::<CreateFeatureFlagError>()
            .unwrap()
        {
            CreateFeatureFlagError::ProjectNotFound => (),
            _ => panic!("Expected ProjectNotFound error"),
        }
    }

    #[test]
    fn test_create_project_if_feature_name_already_exists_in_project() {
        // Arrange
        let mut mock_project_repo = MockProjectRepository::new();
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let organization_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();

        mock_project_repo
            .expect_get()
            .with(eq(project_uuid))
            .times(1)
            .returning(move |_| {
                Ok(Some(Project::new(
                    project_uuid,
                    organization_uuid,
                    "feature-x".to_string(),
                    None,
                )))
            });

        mock_feature_flag_repo
            .expect_name_exists()
            .with(eq("feature-x"), eq(project_uuid))
            .returning(|_, _| Ok(true));

        let use_case = CreateFeatureFlagUseCase::new(&mock_project_repo, &mock_feature_flag_repo);

        // Act
        let result = use_case.execute("feature-x".to_string(), project_uuid, organization_uuid);

        // Assert
        assert!(result.is_err());
        match *result
            .unwrap_err()
            .downcast::<CreateFeatureFlagError>()
            .unwrap()
        {
            CreateFeatureFlagError::NameExists => (),
            _ => panic!("Expected ProjectNotFound error"),
        }
    }

    #[test]
    fn test_create_project_if_feature_on_success() {
        // Arrange
        let mut mock_project_repo = MockProjectRepository::new();
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let organization_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();

        mock_project_repo
            .expect_get()
            .with(eq(project_uuid))
            .times(1)
            .returning(move |_| {
                Ok(Some(Project::new(
                    project_uuid,
                    organization_uuid,
                    "feature-x".to_string(),
                    None,
                )))
            });

        mock_feature_flag_repo
            .expect_name_exists()
            .with(eq("feature-x"), eq(project_uuid))
            .returning(|_, _| Ok(false));
        mock_feature_flag_repo.expect_add().returning(|_| Ok(()));
        let use_case = CreateFeatureFlagUseCase::new(&mock_project_repo, &mock_feature_flag_repo);

        // Act
        let result = use_case
            .execute("feature-x".to_string(), project_uuid, organization_uuid)
            .unwrap();

        // Assert
        assert_eq!(result.is_enabled, false);
        assert_eq!(result.project_id, project_uuid);
        assert_eq!(result.name, "feature-x".to_string());
    }
}
