use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use std::time::Duration;

pub struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn interval(&self, #[arg(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(Duration::from_secs(1)).map(move |_| {
            value += step;
            value
        })
    }
}
