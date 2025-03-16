// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        content -> Text,
        is_done -> Bool,
    }
}
