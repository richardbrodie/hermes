table! {
    feed_channels (id) {
        id -> Int4,
        title -> Varchar,
        link -> Varchar,
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
    }
}

joinable!(feed_items -> feed_channels (feed_channel_id));

allow_tables_to_appear_in_same_query!(
    feed_channels,
    feed_items,
);
