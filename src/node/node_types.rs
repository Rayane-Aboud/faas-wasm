use std::{collections::HashMap, sync::Arc};

use tokio::time::Instant;

use crate::task::Task;

/*structure of the node when the app is up*/
pub trait Node{
    fn is_alive(&self)-> bool;
    fn load_summary(&self)->(f32,f32,Option<f32>);
    fn sync_info(&mut self);
}
/*pub struct Node {
  pub id: Arc<str>,
  pub url: Arc<String>,
  pub status: NodeStatus,
  pub resources: Resources,
  pub tasks: HashMap<Arc<str>, Task>
}*/

#[derive(PartialEq)]
pub enum NodeStatus {
  Alive,
  Overloaded,
  Dead, 
}
pub enum TaskStatus {
  Running,
  Pending,
}
pub struct InMemoryNode{
  pub name: Arc<str>,
  pub tasks: HashMap<Arc<str>, Task>,
  pub status: NodeStatus,
  pub current_task_status: TaskStatus,
  pub resources: Resources,
  pub last_heartbeat_timestamp: Instant
}

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

struct OsProcessNode {
  pub pid:Arc<str>, 
  pub tasks: HashMap<Arc<str>, Task>,
  pub resources: Resources,
  pub last_heartbeat_timestamp: Instant
}

struct ContainerNode{

}
struct VirtualMachineNode{
  
}
struct NetworkNode{

}
pub enum ThreadStatus {
  Running,
  Panicked
}
#[derive(Clone, Copy)]
pub struct Resources {
    pub cpu_total: u32,
    pub cpu_used: u32,
    pub mem_total: u32,
    pub mem_used: u32,
    pub gpu_total: Option<u32>,
    pub gpu_used: Option<u32>,
}

impl Resources {
  pub fn cpu_usage_percentage(&self) -> f32 {
    self.cpu_used as f32 / self.cpu_total as f32 * 100.0
  }
  
  pub fn mem_usage_percentage(&self) -> f32 {
    self.mem_used as f32 / self.mem_total as f32 * 100.0
  }

  pub fn gpu_usage_percentage(&self) -> Option<f32> {
    match (self.gpu_total, self.gpu_used) {
        (Some(total), Some(used)) if total > 0 => Some(used as f32 / total as f32 * 100.0),
        _ => None,
    }
  }
}