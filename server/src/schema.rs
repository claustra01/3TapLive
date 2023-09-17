// @generated automatically by Diesel CLI.

diesel::table! {
  channels (name) {
      #[max_length = 100]
      name -> Varchar,
      #[max_length = 400]
      token -> Varchar,
      #[max_length = 100]
      title -> Varchar,
      #[max_length = 100]
      owner -> Varchar,
      created_at -> Timestamp,
      updated_at -> Timestamp,
  }
}

diesel::table! {
  comments (id) {
      id -> Int4,
      #[max_length = 400]
      body -> Varchar,
      #[max_length = 100]
      channel -> Varchar,
      #[max_length = 100]
      owner -> Varchar,
      created_at -> Timestamp,
      updated_at -> Timestamp,
  }
}

diesel::allow_tables_to_appear_in_same_query!(
  channels,
  comments,
);
