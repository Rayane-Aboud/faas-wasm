use std::{collections::HashMap, sync::Arc};

use tokio::time::Instant;

use crate::{node::{enums::{NodeStatus, TaskStatus, ThreadStatus}, resources::Resources}, task::Task};

struct ThreadBasedNode{
  //thread handle,
  pub thread_id:Arc<str>,
  pub tasks: HashMap<Arc<str>, Task>,
  pub status: NodeStatus,
  pub current_task_status: TaskStatus,
  pub thread_status: ThreadStatus,
  pub resources: Resources,
  pub last_heartbeat_timestamp: Instant
}