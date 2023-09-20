use async_graphql::Subscription;
use futures_util::Stream;

use crate::RECEIVER;

use crate::objects::gql_objects;

pub struct Subscription;

#[Subscription]
impl Subscription {
  async fn comments(&self) -> impl Stream<Item = gql_objects::Comment> {
    async_stream::stream! {
      loop {
          let mut rx = RECEIVER.get().unwrap().lock().await;
          if let Some(item) = (*rx).recv().await {
              yield item;
          }
      }
    }
  }
}