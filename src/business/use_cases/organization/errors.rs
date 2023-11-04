#[derive(Debug)]
pub enum CreateOrganizationError {
    EmailExists,
}

impl std::fmt::Display for CreateOrganizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CreateOrganizationError::EmailExists => {
                write!(f, "An organization with the given email already exists.")
            }
        }
    }
}

impl std::error::Error for CreateOrganizationError {}
