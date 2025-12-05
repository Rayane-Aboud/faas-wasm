#[derive(PartialEq)]
pub enum NodeStatus {
  Alive,
  Overloaded,
  Degraded,
  Dead, 
}
pub enum Reachability {
    Reachable
}

pub enum TaskAssignmentChannel {
    Http,
    GRpc,
    MessageQueue
}
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