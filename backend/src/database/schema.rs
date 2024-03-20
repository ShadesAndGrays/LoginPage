// @generated automatically by Diesel CLI.

diesel::table! {
    user (name) {
        #[max_length = 45]
        name -> Varchar,
        #[max_length = 45]
        password -> Varchar,
    }
}
