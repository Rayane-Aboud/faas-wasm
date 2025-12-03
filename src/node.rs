use std::{collections::HashMap, sync::Arc};

use crate::task::Task;

pub struct Node {
  pub id: Arc<str>,
  //-------------------------------
  pub url: Arc<str>,
  pub status: NodeStatus,
  //-------------------------------
  pub resources: Resources,
  pub resources_usage_percentage: Resources,
  //-------------------------------
  pub tasks: HashMap<Arc<str>, Task>
}

pub struct Resources {
  cpu: u32,
  mem: u32,
  gpu: Option<u32>
}

impl Node {
  pub fn is_alive(&self)-> bool {
    self.status == NodeStatus::Alive
  }

  pub fn load_summary(&self) {

  }
}

pub struct NodeDto{
    pub url: Arc<str>,
    pub status: NodeStatus,
}

#[derive(PartialEq)]
pub enum NodeStatus {
  Alive,
  Overloaded,
  Dead, 
}

