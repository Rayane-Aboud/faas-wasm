use std::sync::Arc;

use sqlx::Sqlite;

pub struct NodeRepository {
    pool: Sqlite,
}

impl NodeRepository{
    pub fn new(pool: Sqlite) -> Self {
        Self {pool}
    }
    pub async fn get(&self){}
    pub async fn list(&self){}
    pub async fn insert(&self){}
    pub async fn update(&self){}
    pub async fn delete(&self){}
}