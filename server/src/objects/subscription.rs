use std::time::Duration;

use async_graphql::Subscription;
use futures_util::Stream;
use tokio_stream::StreamExt;

pub struct Subscription;

#[Subscription]
impl Subscription {
  async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
    println!("hello, subscription");
    let mut value = 0;
    tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
      .map(move |_| {
        value += step;
        value
      })
  }
}