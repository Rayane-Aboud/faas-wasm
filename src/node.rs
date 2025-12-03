pub struct Node {
  pub id: Arc<str>,
  pub url: Arc<str>,
  pub status: NodeStatus,
}

pub struct NodeDto{
    pub url: Arc<str>,
    pub status: NodeStatus,
}

pub enum NodeStatus {
  Alive,
  Overloaded,
  Dead, 
}

