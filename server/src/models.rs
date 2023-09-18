use chrono::NaiveDateTime;
use serde::Serialize;
use diesel::prelude::{Queryable, Selectable, Insertable};

#[derive(Serialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = "channels")]
pub struct Channel {
    name: String, // uuid
    token: String,
    title: String,
    owner_name: String, // display name
    owner_icon: String, // image url
    created_at: NaiveDateTime,
    updated_ad: NaiveDateTime,
}

#[derive(Insertable)]
pub struct NewChannel<'a> {
    name: &'a String, // uuid
    token: &'a String,
    title: &'a String,
    owner_name: &'a String, // display name
    owner_icon: &'a String, // image url
}

#[derive(Serialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = "comments")]
pub struct Comment {
    id: i32, // auto increment
    body: String,
    channel: String, // uuid
    owner: String, // display name
    created_at: NaiveDateTime,
    updated_ad: NaiveDateTime,
}

#[derive(Insertable)]
pub struct NewChannel<'a> {
    body: &'a String,
    channel: &'a String, // uuid
    owner: &'a String, // display name
}
