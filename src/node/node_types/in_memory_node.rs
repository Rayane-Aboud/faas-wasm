use std::{collections::HashMap, sync::Arc};
use tokio::time::Instant;

use crate::{node::{enums::{NodeStatus, TaskStatus}, resources::Resources}, task::Task};


pub struct InMemoryNode{
  pub name: Arc<str>,
  pub tasks: HashMap<Arc<str>, Task>,
  pub status: NodeStatus,
  pub current_task_status: TaskStatus,
  pub resources: Resources,
  pub last_heartbeat_timestamp: Instant
}
