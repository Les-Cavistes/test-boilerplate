// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        description -> Varchar,
        completed -> Bool,
    }
}
