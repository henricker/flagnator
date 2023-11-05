use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectDiesel {
    pub id: String,
    pub organization_id: String,
    pub name: String,
}
