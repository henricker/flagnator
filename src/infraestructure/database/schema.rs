// @generated automatically by Diesel CLI.

diesel::table! {
    organizations (id) {
        id -> Text,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
