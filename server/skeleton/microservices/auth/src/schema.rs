table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        last_login -> Timestamp,
    }
}

table! {
    user_friend (id) {
        id -> Int4,
        from_user_id -> Int4,
        to_friend_id -> Int4,
        status -> Int2,
        requested -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        wallet_address -> Varchar, 
        access_token -> Varchar,
        access_level -> Int2,
        is_blocked -> Int2,
        phone_number -> Varchar,
        email -> Varchar,
        device_id -> Varchar,
        firebase_id -> Nullable<Varchar>,
        prof_img -> Nullable<Varchar>,
        coins -> Int4,
        sex -> Bpchar,
        age -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(
    login_history,
    user_friend,
    users,
);
