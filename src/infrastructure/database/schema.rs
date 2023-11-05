// @generated automatically by Diesel CLI.

diesel::table! {
    organizations (id) {
        id -> Text,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        organization_id -> Text,
        name -> Varchar,
    }
}

diesel::joinable!(projects -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(organizations, projects,);
