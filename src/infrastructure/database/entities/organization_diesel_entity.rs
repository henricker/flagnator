use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrganizationDiesel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
