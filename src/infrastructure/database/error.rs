#[derive(Debug)]
pub enum DatabaseError {
    GetOrganizationError,
    GetProjectError,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DatabaseError::GetOrganizationError => {
                write!(f, "Problem in database on returning Organization")
            }
            DatabaseError::GetProjectError => {
                write!(f, "Problem in database on returning Project")
            }
        }
    }
}

impl std::error::Error for DatabaseError {}
