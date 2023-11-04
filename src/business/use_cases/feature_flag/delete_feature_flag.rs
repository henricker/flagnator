use std::error::Error;

use uuid::Uuid;

use crate::{
    business::repositories::feature_flag_repository::FeatureFlagRepository,
    domain::entities::feature_flag::{self, FeatureFlag},
};

use super::errors::DeleteFeatureFlagError;

pub struct DeleteFeatureFlagUseCase<'a> {
    feature_flag_repository: &'a dyn FeatureFlagRepository,
}

impl<'a> DeleteFeatureFlagUseCase<'a> {
    pub fn new(feature_flag_repository: &'a dyn FeatureFlagRepository) -> Self {
        DeleteFeatureFlagUseCase {
            feature_flag_repository,
        }
    }

    pub fn execute(&self, feature_flag_id: Uuid, project_id: Uuid) -> Result<bool, Box<dyn Error>> {
        let feature_flag_opt = self
            .feature_flag_repository
            .get(feature_flag_id, project_id)?;

        let feature_flag = match feature_flag_opt {
            None => return Err(Box::new(DeleteFeatureFlagError::FeatureNotFound)),
            Some(feature_flag) => feature_flag,
        };

        if feature_flag.project_id != project_id {
            return Err(Box::new(
                DeleteFeatureFlagError::FeatureNotBelongsThisProject,
            ));
        }

        self.feature_flag_repository.delete(feature_flag_id)?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::business::repositories::feature_flag_repository::MockFeatureFlagRepository;
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test_delete_feature_flag_not_exists() {
        // Arrange
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let project_uuid = Uuid::new_v4();
        let feature_flag_uuid = Uuid::new_v4();

        mock_feature_flag_repo
            .expect_get()
            .returning(|_, _| Ok(None));

        let use_case = DeleteFeatureFlagUseCase::new(&mock_feature_flag_repo);

        // Act
        let result = use_case.execute(feature_flag_uuid, project_uuid);

        // Assert
        assert!(result.is_err());
        match *result
            .unwrap_err()
            .downcast::<DeleteFeatureFlagError>()
            .unwrap()
        {
            DeleteFeatureFlagError::FeatureNotFound => (),
            _ => panic!("Invalid error FeatureNotBelongsThisProject"),
        }
    }

    #[test]
    fn test_delete_feature_flag_not_belongs_project() {
        // Arrange
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let project_uuid = Uuid::new_v4();
        let another_project_uuid = Uuid::new_v4();
        let feature_flag_uuid = Uuid::new_v4();

        mock_feature_flag_repo.expect_get().returning(move |_, _| {
            Ok(Some(FeatureFlag {
                id: feature_flag_uuid,
                project_id: another_project_uuid,
                name: "feature-x".to_string(),
                is_enabled: true,
                schedule: None,
            }))
        });

        let use_case = DeleteFeatureFlagUseCase::new(&mock_feature_flag_repo);

        // Act
        let result = use_case.execute(feature_flag_uuid, project_uuid);

        // Assert
        assert!(result.is_err());
        match *result
            .unwrap_err()
            .downcast::<DeleteFeatureFlagError>()
            .unwrap()
        {
            DeleteFeatureFlagError::FeatureNotBelongsThisProject => (),
            _ => panic!("Invalid error FeatureNotBelongsThisProject"),
        }
    }

    fn test_delete_feature_flag_on_success() {
        // Arrange
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let project_uuid = Uuid::new_v4();
        let feature_flag_uuid = Uuid::new_v4();

        mock_feature_flag_repo.expect_get().returning(move |_, _| {
            Ok(Some(FeatureFlag {
                id: feature_flag_uuid,
                project_id: project_uuid,
                name: "feature-x".to_string(),
                is_enabled: true,
                schedule: None,
            }))
        });

        let use_case = DeleteFeatureFlagUseCase::new(&mock_feature_flag_repo);

        // Act
        let result = use_case.execute(feature_flag_uuid, project_uuid).unwrap();

        // Assert
        assert!(result)
    }
}
