pub trait Node{
    fn is_alive(&self)-> bool;
    fn load_summary(&self)->(f32,f32,Option<f32>);
    fn sync_info(&mut self);
}