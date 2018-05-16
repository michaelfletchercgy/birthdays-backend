table! {
    birthdays (id) {
        id -> Int4,
        title -> Varchar,
        year -> Nullable<Int4>,
        month -> Int4,
        day -> Int4,
        user_id -> Int4,
    }
}

table! {
    subscriptions (subscription_id) {
        subscription_id -> Int4,
        user_id -> Int4,
        url -> Text,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Text,
    }
}

joinable!(birthdays -> users (user_id));
joinable!(subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    birthdays,
    subscriptions,
    users,
);