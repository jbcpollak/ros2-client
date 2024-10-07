use async_trait::async_trait;

use rustdds::dds::WriteResult;
use serde::Serialize;

use crate::{gid::Gid, node::Node};

#[async_trait]
pub trait Publisher<M>: Send + Sync
where
  M: Serialize + Send + 'static + Sync,
{
  fn publish(&self, message: M) -> WriteResult<(), M>;

  fn assert_liveliness(&self) -> WriteResult<(), ()>;

  fn guid(&self) -> rustdds::GUID;

  fn gid(&self) -> Gid;

  fn get_subscription_count(&self, my_node: &Node) -> usize;

  async fn wait_for_subscription(&self, my_node: &Node) -> ();

  async fn async_publish(&self, message: M) -> WriteResult<(), M>;
}
