use std::{
    collections::HashMap, 
    sync::Arc
};

use crate::node::{Node};

pub struct Supervisor {
  pub nodes: HashMap<
    Arc<str>, //node id
    Node  //node
  >
}

impl Supervisor {
  pub fn new()-> Supervisor{
    Supervisor {
      nodes: HashMap::<Arc<str>,Node>::new()
    }
  }
  
  /* monitoring */
  //heartbeats
  pub fn sync_nodes_info(&mut self){
    for (_key,node) in self.nodes.iter_mut() {
      node.sync_info()
    }
  }

  pub fn check_node_heartbeat(&self, id: &str) -> bool {
    if let Some(node) = self.nodes.get(id){
      return node.is_alive();
    }
    false
  }

  pub fn get_node_health_summary(&self, id: &str) -> (f32,f32,Option<f32>) {
    if let Some(node) = self.nodes.get(id){
      return node.load_summary();
    }
    (0.0f32,0.0f32,None)
  }
  //death signal

  
  /*
  pub fn list_nodes(&self)-> HashMap<Arc<str>,Node>{
    self.nodes
  }

  pub fn create_node(&self, node_data: &NodeDto){

  }

  pub fn update_node(
    &self, 
    id: Arc<str>,
    node_data: &NodeDto
  ){

  }

  pub fn delete_node(&self, id:Arc<str>){

  }
   */
}

