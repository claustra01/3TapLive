use chrono::NaiveDateTime;
use async_graphql::SimpleObject;
use serde::Serialize;
use diesel::prelude::{Queryable, Selectable, Insertable};

#[derive(SimpleObject, Serialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = "channels")]
struct Channel {
    name: String, // uuid
    token: String,
    title: String,
    owner: String, // display name
    created_at: NaiveDateTime,
    updated_ad: NaiveDateTime,
}

#[derive(Insertable)]
struct NewChannel<'a> {
    name: &'a String, // uuid
    token: &'a String,
    title: &'a String,
    owner: &'a String, // display name
}

#[derive(SimpleObject, Serialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = "comments")]
struct Comment {
    id: i32, // auto increment
    body: String,
    channel: String, // uuid
    owner: String, // display name
    created_at: NaiveDateTime,
    updated_ad: NaiveDateTime,
}

#[derive(Insertable)]
struct NewChannel<'a> {
    body: &'a String,
    channel: &'a String, // uuid
    owner: &'a String, // display name
}
