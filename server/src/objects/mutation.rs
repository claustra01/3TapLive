use async_graphql::Object;

pub struct Mutation;

#[Object]
impl Mutation {

  async fn create_channel(
    &self,
    title: String,
    owner_name: String,
    owner_icon: String
  ) -> super::object::Channel {
    super::object::Channel {
      name: String::from(""),
      token: String::from(""),
      title,
      owner_name,
      owner_icon,
    }
  }
  
}