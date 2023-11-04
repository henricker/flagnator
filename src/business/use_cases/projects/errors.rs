#[derive(Debug)]
pub enum CreateProjectError {
    NameExists,
}

impl std::fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CreateProjectError::NameExists => write!(f, "A project with the given name already exists."),
        }
    }
}

impl std::error::Error for CreateProjectError {}