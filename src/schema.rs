table! {
    account (id) {
        id -> Int4,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        is_admin -> Bool,
    }
}

table! {
    twoot (id) {
        id -> Int4,
        user_id -> Int4,
        content -> Text,
        uuid -> Text,
    }
}

joinable!(twoot -> account (user_id));

allow_tables_to_appear_in_same_query!(account, twoot,);
