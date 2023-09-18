use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Channel {
    name: String, // uuid
    token: String,
    title: String,
    owner_name: String, // display name
    owner_icon: String, // image url
}

#[derive(SimpleObject)]
pub struct Comment {
    id: i32, // auto increment
    body: String,
    channel: String, // uuid
    owner: String, // display name
}
