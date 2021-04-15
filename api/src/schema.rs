table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        expire_at -> Timestamp,
        token -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(access_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(access_tokens, users,);
