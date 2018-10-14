table! {
    documents (id) {
        id -> Int4,
        title -> Text,
    }
}

table! {
    revisions (id) {
        id -> Int4,
        document_id -> Int4,
        created -> Timestamp,
        contents -> Text,
    }
}

joinable!(revisions -> documents (document_id));

allow_tables_to_appear_in_same_query!(
    documents,
    revisions,
);
