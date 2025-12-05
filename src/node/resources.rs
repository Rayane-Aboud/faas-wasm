#[derive(Clone, Copy)]
pub struct Resources {
    pub cpu_total: u32,
    pub cpu_used: u32,
    pub mem_total: u32,
    pub mem_used: u32,
    pub gpu_total: Option<u32>,
    pub gpu_used: Option<u32>,
    pub queue_info: QueueInfo,
}

impl Resources {
  pub fn cpu_usage_percentage(&self) -> f32 {
    self.cpu_used as f32 / self.cpu_total as f32 * 100.0
  }
  
  pub fn mem_usage_percentage(&self) -> f32 {
    self.mem_used as f32 / self.mem_total as f32 * 100.0
  }

  pub fn gpu_usage_percentage(&self) -> Option<f32> {
    match (self.gpu_total, self.gpu_used) {
        (Some(total), Some(used)) if total > 0 => Some(used as f32 / total as f32 * 100.0),
        _ => None,
    }
  }

  pub fn queue_usage_percentage(&self) -> f32 {
    (self.queue_info.pending + self.queue_info.in_flight) as f32 / self.queue_info.max_capacity as f32 * 100.0f32
  }
}

#[derive(Clone, Copy)]
pub struct QueueInfo {
    pub pending: u32,
    pub in_flight: u32,
    pub max_capacity: u32,
}