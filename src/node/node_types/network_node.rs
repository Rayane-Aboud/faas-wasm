use std::{sync::Arc, time::Duration};

use tokio::time::Instant;

use crate::node::{enums::{ErrorState, NodeStatus, Reachability, TaskAssignmentChannel}, node_behavior::Node, resources::Resources};

/* 
TODO:
[x] Last heartbeat timestamp
[ ] Reachability state
[ ] Load cache
[ ] Protocol settings
[ ] Capability info
[ ] Task result endpoint reference
[ ] Authentication or token
[ ] State hash or revision marker
*/
pub struct TaskChannel{
    pub task_assignment_channel: TaskAssignmentChannel,
    //task_result_channel: supervisor retrieves output or error
}
pub enum ProtocolType{
    Http,
    GRpc,
    WebSocket,
    TCP
}
pub struct ProtocolConfig{
    pub protocol_type: ProtocolType,
    //endpoint / port
    pub timeout: u32,
    pub retry: u32,
    //security 
    //auth-tokens
    //header
    //keepalive,compression...
}

pub struct NetworkNode{
    pub id: Arc<str>,
    pub url: String,
    pub status: NodeStatus,
    pub reachability: bool,//Reachability,/*distinguish between node is alive but busy and node cannot be reached*/
    pub last_heartbeat_timestamp:Instant,
    pub resources: Resources,
    pub error_state: ErrorState,/*supervisor logs it and may mark node degraded or blocked */
    pub task_channel: TaskChannel,/*supervisor uses it to dispatch work */
    pub protocol_config: ProtocolConfig,
    /*supervise liveness. If too old, node treated as dead */
    
    //capability_info
    /*Supported operations, versions, concurrency limits.
Role: supervisor avoids sending incompatible tasks */
    //state_revision
    /*Hash or number representing node’s internal configuration version.
Role: detect mismatch between supervisor’s global state and node’s local state */
}

impl Node for NetworkNode {
    fn is_alive(&self)-> bool {
        let reachability = self.reachability;
        let last_heartbeat_timestamp = self.last_heartbeat_timestamp;
        let delta_time = Instant::now() - last_heartbeat_timestamp;
        if delta_time > Duration::from_millis(1000) || reachability == false  {
            return false;
        }
        true
    }

    fn load_summary(&self)->(f32,f32,f32,Option<f32>) {
        (
            self.resources.cpu_usage_percentage(),
            self.resources.mem_usage_percentage(),
            self.resources.queue_usage_percentage(),
            self.resources.gpu_usage_percentage()
        )
    }

    fn sync_info(&mut self) {
        todo!()
    }
}