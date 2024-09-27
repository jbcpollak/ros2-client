use std::{future::Future, pin::Pin};

use rustdds::dds::WriteResult;
use serde::Serialize;

use crate::{gid::Gid, node::Node};

pub trait Publisher<M: Serialize + Send> {
  fn publish(&self, message: M) -> WriteResult<(), M>;

  fn assert_liveliness(&self) -> WriteResult<(), ()>;

  fn guid(&self) -> rustdds::GUID;

  fn gid(&self) -> Gid;

  fn get_subscription_count(&self, my_node: &Node) -> usize;

  fn wait_for_subscription(&self, my_node: &Node) -> Pin<Box<dyn Future<Output = ()> + Send>>;

  fn async_publish(&self, message: M) -> Pin<Box<dyn Future<Output = WriteResult<(), M>> + Send>>;
}
