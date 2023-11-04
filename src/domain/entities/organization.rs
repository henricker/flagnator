use uuid::Uuid;


#[derive(Debug)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String
}

impl Organization {
    // Constructor and other methods for the Organization entity
    pub fn new(id: Uuid, name: String, email: String, password: String) -> Organization {
        Organization { id, name, email, password }
    }
}