use async_graphql::Object;

pub struct Mutation;

#[Object]
impl Mutation {
  async fn echo(&self, message: String) -> String {
    message
  }
}