#[async_trait::async_trait]
pub trait Node{
    fn is_alive(&self)-> bool;
    fn load_summary(&self)->(f32,f32,f32,Option<f32>);
    async fn sync_info(&mut self)-> anyhow::Result<()>;
}