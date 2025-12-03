use std::{
    collections::HashMap, 
    sync::Arc
};

use crate::node::{self, NodeDto};

pub struct Supervisor {
  pub nodes: HashMap<
    Arc<str>, //node id
    Node  //node
  >
}

impl Supervisor {
  pub fn new(){
    Supervisor {
      nodes: HashMap::<Arc<str>,Node>::new()
    }
  }
  
  //heartbeats
  pub fn check_node_heartbeat(&self, id: &str)->bool{
    if let Some(node) = self.nodes.get(id){
        node.is_alive()
    }
    false
  }





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

