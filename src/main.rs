use std::{collections::HashMap, sync::Arc};

// src/main.rs
use anyhow::Result;

pub struct Supervisor {
  pub nodes: HashMap<
    Arc<str>, //node id
    Node  //node
  >
}

impl Supervisor {
  pub fn new(){
    Supervisor {
      nodes: HashMap()
    }
  }
  pub fn monitor_health(){

  }
}

pub struct Node {
  pub id: Arc<str>,
  pub url: Arc<str>,
  pub status: Status,
}
pub enum Status {
  Alive,
  Overloaded,
  Dead, 
}

fn main() -> Result<()> {
  
  Ok(())
}

/* 
scheduler that runs wasm files while knowing their resources

*/