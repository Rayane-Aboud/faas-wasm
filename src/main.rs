// src/main.rs
use anyhow::Result;

pub struct Scheduler {
  pub workers: Vec<Worker>,
  pub tasks: Vec<Task>
}

//
pub trait Resource{}
//
pub struct Cpu;
impl Resource for Cpu {}
//
pub struct Ram;
impl Resource for Ram{}

pub struct Worker {
  pub task: Task,

}
pub struct Task;
impl Future for Task {
    type Output;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

fn main() -> Result<()> {
  
  Ok(())
}

/* 
scheduler that runs wasm files while knowing their resources

*/