table! {
    accounts (id) {
        id -> Integer,
        ip_addr -> Varchar,
        token -> Varchar,
    }
}

table! {
    owo_faces (id) {
        id -> Integer,
        face -> Varchar,
        face_size -> Integer,
        style -> Varchar,
        emotion -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    owo_faces,
);
