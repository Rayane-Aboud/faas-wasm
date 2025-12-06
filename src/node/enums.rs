use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
#[derive(Serialize,Deserialize)]
pub enum NodeStatus {
  Alive,
  Overloaded,
  Degraded,
  Dead, 
}
pub enum Reachability {
    Reachable
}
#[derive(Serialize,Deserialize)]
pub enum TaskAssignmentChannel {
    Http,
    GRpc,
    MessageQueue
}

#[derive(Serialize,Deserialize)]
pub enum ErrorState {
    Default
}

pub enum TaskStatus {
  Running,
  Pending,
}

pub enum ThreadStatus {
  Running,
  Panicked
}