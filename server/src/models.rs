use chrono::NaiveDateTime;
use serde::Serialize;
use diesel::prelude::{Queryable, Selectable, Insertable};

use crate::schema::*;

#[derive(Serialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = channels)]
pub struct Channel {
    pub name: String, // uuid
    pub token: String,
    pub title: String,
    pub owner_name: String, // display name
    pub owner_icon: String, // image url
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel<'a> {
    pub name: &'a String, // uuid
    pub token: &'a String,
    pub title: &'a String,
    pub owner_name: &'a String, // display name
    pub owner_icon: &'a String, // image url
}

#[derive(Serialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32, // auto increment
    pub body: String,
    pub channel: String, // uuid
    pub owner: String, // display name
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a> {
    pub body: &'a String,
    pub channel: &'a String, // uuid
    pub owner: &'a String, // display name
}
