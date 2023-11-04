#[derive(Debug)]
pub enum CreateFeatureFlagError {
    ProjectNotFound,
    NameExists
}

#[derive(Debug)]
pub enum DeleteFeatureFlagError {
    FeatureNotFound,
    FeatureNotBelongsThisProject
}

impl std::fmt::Display for CreateFeatureFlagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CreateFeatureFlagError::ProjectNotFound => write!(f, "Project not found"),
            CreateFeatureFlagError::NameExists => write!(f, "feature name already exists")
        }
    }
}

impl std::fmt::Display for DeleteFeatureFlagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DeleteFeatureFlagError::FeatureNotFound => write!(f, "Feature not found"),
            DeleteFeatureFlagError::FeatureNotBelongsThisProject => write!(f, "Feature not belongs this project")
        }
    }
}

impl std::error::Error for DeleteFeatureFlagError {}

impl std::error::Error for CreateFeatureFlagError {}
