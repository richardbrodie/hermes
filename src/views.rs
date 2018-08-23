table! {
    subscribed_feeds_with_count_view (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        site_link -> Varchar,
        feed_link -> Varchar,
        updated_at -> Timestamptz,
        user_id -> Int4,
        unseen_count -> Int4,
    }
}

table! {
    subscribed_items_view (id) {
        id -> Int4,
        guid -> Varchar,
        link -> Varchar,
        title -> Varchar,
        summary -> Nullable<Text>,
        content -> Nullable<Text>,
        published_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        feed_id -> Int4,
        subscribed_item_id -> Int4,
        user_id -> Int4,
        seen -> Bool,
    }
}
