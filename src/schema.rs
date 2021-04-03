table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        deadline -> Nullable<Date>,
        completed -> Nullable<Timestamp>,
    }
}
