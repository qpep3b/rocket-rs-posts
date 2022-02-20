table! {
    posts (post_id) {
        post_id -> Int4,
        title -> Varchar,
        content -> Text,
        author_id -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        login -> Varchar,
        secret_password -> Varchar,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
