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
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(feed_items -> feed_channels (feed_channel_id));

allow_tables_to_appear_in_same_query!(
    feed_channels,
    feed_items,
    users,
);
