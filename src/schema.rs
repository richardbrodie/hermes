table! {
    feeds (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        site_link -> Varchar,
        feed_link -> Varchar,
        updated_at -> Timestamptz,
    }
}

table! {
    items (id) {
        id -> Int4,
        guid -> Varchar,
        link -> Varchar,
        title -> Varchar,
        summary -> Nullable<Text>,
        content -> Nullable<Text>,
        published_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        feed_id -> Int4,
    }
}

table! {
    subscribed_feeds (id) {
        id -> Int4,
        user_id -> Int4,
        feed_id -> Int4,
    }
}

table! {
    subscribed_items (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
        seen -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Bytea,
    }
}

joinable!(items -> feeds (feed_id));
joinable!(subscribed_feeds -> feeds (feed_id));
joinable!(subscribed_feeds -> users (user_id));
joinable!(subscribed_items -> items (item_id));
joinable!(subscribed_items -> users (user_id));

allow_tables_to_appear_in_same_query!(
    feeds,
    items,
    subscribed_feeds,
    subscribed_items,
    users,
);
