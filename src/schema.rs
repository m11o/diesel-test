// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
