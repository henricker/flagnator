use std::error::Error;

use uuid::Uuid;

use crate::{business::repositories::feature_flag_repository::{FeatureFlagRepository, UpdateFeatureFlag}, domain::entities::feature_flag::{FeatureFlag, self}};

use super::errors::UpdateFeatureFlagError;




pub struct UpdateFeatureFlagUseCase<'a> {
    feature_flag_repository: &'a dyn FeatureFlagRepository
}

impl<'a> UpdateFeatureFlagUseCase<'a> {
    pub fn new(feature_flag_repository: &'a dyn FeatureFlagRepository) -> Self {
        UpdateFeatureFlagUseCase { 
            feature_flag_repository
        }
    }

    pub fn execute(&self, feature_flag_id: Uuid, project_id: Uuid, feature_flag_data_to_update: UpdateFeatureFlag) -> Result<FeatureFlag, Box<dyn Error>> {
        let feature_flag_opt = self.feature_flag_repository.get(feature_flag_id, project_id)?;

        let feature_flag = match feature_flag_opt {
            None => return Err(Box::new(UpdateFeatureFlagError::FeatureNotFound)),
            Some(feature_flag) => feature_flag
        };

        if feature_flag.project_id != project_id {
            return Err(Box::new(UpdateFeatureFlagError::FeatureNotBelongsThisProject))
        }

        let result = self.feature_flag_repository.update(feature_flag_id, feature_flag_data_to_update)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use crate::{business::repositories::feature_flag_repository::MockFeatureFlagRepository, domain::entities::project::Project};

    use super::*;

    #[test]
    fn test_update_feature_flag_if_not_exists_feature() {
        // Arrange
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let feature_flag_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();

        mock_feature_flag_repo.expect_get().returning(|_,_| Ok(None));

        let use_case = UpdateFeatureFlagUseCase::new(&mock_feature_flag_repo);
        
        // Act
        let result = use_case.execute(feature_flag_uuid, project_uuid, UpdateFeatureFlag { name: Some("feature-y".to_string()), is_enabled: Some(true) });

        // Assert
        assert!(result.is_err());
        match *result.unwrap_err().downcast::<UpdateFeatureFlagError>().unwrap() {
            UpdateFeatureFlagError::FeatureNotFound => (),
            _ => panic!("Expected FeatureNotFound error"),
        }
    }

    #[test]
    fn test_update_feature_flag_if_not_belongs_the_project() {
        // Arrange
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let feature_flag_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();
        let other_project_uuid = Uuid::new_v4();

        mock_feature_flag_repo.expect_get().returning(move |_,_| Ok(Some(
            FeatureFlag { id: feature_flag_uuid, project_id: other_project_uuid, name: "feature-x".to_string(), is_enabled: true, schedule: None }
        )));

        let use_case = UpdateFeatureFlagUseCase::new(&mock_feature_flag_repo);
        
        // Act
        let result = use_case.execute(feature_flag_uuid, project_uuid, UpdateFeatureFlag { name: Some("feature-y".to_string()), is_enabled: Some(true) });

        // Assert
        assert!(result.is_err());
        match *result.unwrap_err().downcast::<UpdateFeatureFlagError>().unwrap() {
            UpdateFeatureFlagError::FeatureNotBelongsThisProject => (),
            _ => panic!("Expected FeatureNotFound error"),
        }
    }

    #[test]
    fn test_update_feature_flag_on_success() {
        // Arrange
        let mut mock_feature_flag_repo = MockFeatureFlagRepository::new();

        let feature_flag_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();

        mock_feature_flag_repo.expect_get().returning(move |_,_| Ok(Some(
            FeatureFlag { id: feature_flag_uuid, project_id: project_uuid, name: "feature-x".to_string(), is_enabled: true, schedule: None }
        )));

        mock_feature_flag_repo.expect_update().returning(move |_,_| Ok(FeatureFlag {
            id: feature_flag_uuid,
            is_enabled: true,
            name: "feature-y".to_string(),
            project_id: project_uuid,
            schedule: None
        }));

        let use_case = UpdateFeatureFlagUseCase::new(&mock_feature_flag_repo);
        
        // Act
        let result = use_case.execute(feature_flag_uuid, project_uuid, UpdateFeatureFlag { name: Some("feature-y".to_string()), is_enabled: Some(true) }).unwrap();

        // Assert
        assert_eq!(result.name, "feature-y");
        assert_eq!(result.is_enabled, true);
    }


}