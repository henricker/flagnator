#[derive(Debug)]
pub enum DatabaseError {
    GetOrganizationError,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DatabaseError::GetOrganizationError => {
                write!(f, "Problem in database on returning Organization")
            }
        }
    }
}

impl std::error::Error for DatabaseError {}
