use std::{collections::HashMap, sync::Arc};

use tokio::time::Instant;

use crate::{node::resources::Resources, task::Task};

struct OsProcessNode {
  pub pid:Arc<str>, 
  pub tasks: HashMap<Arc<str>, Task>,
  pub resources: Resources,
  pub last_heartbeat_timestamp: Instant
}