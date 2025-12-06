use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
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
#[derive(Serialize,Deserialize)]
pub struct TaskChannel{
    pub task_assignment_channel: TaskAssignmentChannel,
    //task_result_channel: supervisor retrieves output or error
}
#[derive(PartialEq)]
#[derive(Serialize,Deserialize)]
pub enum ProtocolType{
    Http,
    GRpc,
    WebSocket,
    TCP
}
#[derive(Serialize,Deserialize)]
pub struct ProtocolConfig{
    pub protocol_type: ProtocolType,
    //endpoint / port
    pub timeout: u32,
    pub retry: u32,//
    //security 
    pub auth_tokens: Option<String>,
    pub headers: Option<HashMap<Arc<str>, Arc<str>>>
    //keepalive,compression...
}
#[derive(Serialize,Deserialize)]
pub struct NetworkNode{
    pub id: Arc<str>,
    pub url: String,
    pub status: NodeStatus,
    pub reachability: bool,//Reachability,/*distinguish between node is alive but busy and node cannot be reached*/
    pub last_heartbeat_timestamp:DateTime<Utc>,
    pub resources: Resources,
    pub error_state: Option<ErrorState>,/*supervisor logs it and may mark node degraded or blocked */
    pub task_channel: Option<TaskChannel>,/*supervisor uses it to dispatch work */
    pub protocol_config: ProtocolConfig,
    //capability_info
    /*Supported operations, versions, concurrency limits.
Role: supervisor avoids sending incompatible tasks */
    //state_revision
    /*Hash or number representing node’s internal configuration version.
Role: detect mismatch between supervisor’s global state and node’s local state */
}
#[derive(Serialize, Deserialize)]
pub struct NetworkNodeSyncInfo {
    pub reachability: bool,
    pub resources: Resources,
    pub error_state: Option<ErrorState>
}


impl Node for NetworkNode {
    fn is_alive(&self)-> bool {
        let reachability = self.reachability;
        let last_heartbeat_timestamp = self.last_heartbeat_timestamp;
        let delta_time = Utc::now() - last_heartbeat_timestamp;
        if delta_time > Duration::milliseconds(1000) || reachability == false  {
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

    async fn sync_info(&mut self) -> anyhow::Result<()> {
        if self.protocol_config.protocol_type == ProtocolType::Http{
            let body = reqwest::get(self.url.clone())
                .await?
                .text()
                .await?;
            let node_sync_info: NetworkNodeSyncInfo = serde_json::from_str(&body)?;
            //----------------------------------------
            self.last_heartbeat_timestamp = Utc::now();
            if  node_sync_info.reachability==true {
                self.status = NodeStatus::Alive
            }
            self.reachability = node_sync_info.reachability;
            self.resources = node_sync_info.resources;
            self.error_state = node_sync_info.error_state;
            //----------------------------------------
        }
        Ok(())
    }
}