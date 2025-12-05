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

pub enum ThreadStatus {
  Running,
  Panicked
}