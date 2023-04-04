// @generated automatically by Diesel CLI.

diesel::table! {
    wines (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        year -> Int4,
        price -> Int4,
    }
}
