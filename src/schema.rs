table! {
    feed_channels (id) {
        id -> Int4,
        title -> Varchar,
        site_link -> Varchar,
        feed_link -> Varchar,
        description -> Text,
        updated_at -> Timestamp,
    }
}

table! {
    feed_items (id) {
        id -> Int4,
        guid -> Varchar,
        title -> Varchar,
        link -> Varchar,
        description -> Text,
        published_at -> Timestamp,
        feed_channel_id -> Int4,
        content -> Nullable<Text>,
    }
}

table! {
    subscriptions (user_id, feed_channel_id) {
        user_id -> Int4,
        feed_channel_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Bytea,
    }
}

joinable!(feed_items -> feed_channels (feed_channel_id));
joinable!(subscriptions -> feed_channels (feed_channel_id));
joinable!(subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    feed_channels,
    feed_items,
    subscriptions,
    users,
);
