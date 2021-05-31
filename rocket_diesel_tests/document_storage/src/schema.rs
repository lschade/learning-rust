table! {
    app_user (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

table! {
    collection (id) {
        id -> Int4,
        owner -> Int4,
        name -> Text,
    }
}

table! {
    document (id) {
        id -> Int4,
        owner -> Int4,
        name -> Text,
        filename -> Text,
        collection -> Nullable<Int4>,
        document_type -> Nullable<Text>,
    }
}

joinable!(collection -> app_user (owner));
joinable!(document -> app_user (owner));
joinable!(document -> collection (collection));

allow_tables_to_appear_in_same_query!(
    app_user,
    collection,
    document,
);
