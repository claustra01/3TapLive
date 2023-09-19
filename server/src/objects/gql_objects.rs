use async_graphql::SimpleObject;

#[derive(SimpleObject, Debug, Clone)]
pub struct Channel {
    pub name: String, // uuid
    pub token: String,
    pub title: String,
    pub owner_name: String, // display name
    pub owner_icon: String, // image url
}

#[derive(SimpleObject, Debug, Clone)]
pub struct Comment {
    pub body: String,
    pub channel: String, // uuid
    pub owner: String, // display name
}
