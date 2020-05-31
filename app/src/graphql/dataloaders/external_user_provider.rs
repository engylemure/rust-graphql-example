use crate::graphql::context::ArcDbPool;
use crate::graphql::dataloaders::CachedDataLoader;
use crate::graphql::objects::external_user_provider::ExternalUserProvider;
use crate::graphql::objects::user::User;
use async_trait::async_trait;
use dataloader::BatchFn;
use diesel::prelude::*;
use std::collections::HashMap;

pub type EupByUserId = CachedDataLoader<String, Vec<ExternalUserProvider>, EupByUserIdLoaderFn>;

pub struct EupByUserIdLoaderFn {
    pub db: ArcDbPool,
}

impl EupByUserIdLoaderFn {
    pub fn new(db: ArcDbPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl BatchFn<String, Vec<ExternalUserProvider>> for EupByUserIdLoaderFn {
    async fn load(&self, keys: &[String]) -> HashMap<String, Vec<ExternalUserProvider>> {
        use crate::schema::external_user_providers::dsl::*;
        let conn: &MysqlConnection = &self.db.get().unwrap();
        let data: Vec<ExternalUserProvider> = {
            match external_user_providers
                .filter(user_id.eq_any(keys))
                .load::<ExternalUserProvider>(conn)
            {
                Ok(r) => r,
                Err(_) => Vec::new(),
            }
        };
        let mapped_data: HashMap<String, Vec<ExternalUserProvider>> =
            keys.iter().map(|v| (v.clone(), vec![])).collect();
        data.into_iter().fold(mapped_data, |mut acc, val| {
            if let Some(values) = acc.get_mut(&val.user_id) {
                values.push(val);
            }
            acc
        })
    }
}
