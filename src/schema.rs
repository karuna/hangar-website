table! {
    users (id) {
        id -> Integer,
        email -> Text,
        encrypted_password -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        access_token -> Nullable<Text>,
        last_access -> Timestamp,
    }
}

