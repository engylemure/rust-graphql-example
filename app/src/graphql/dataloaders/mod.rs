pub mod external_user_provider;

use crate::graphql::context::ArcDbPool;
use dataloader::cached::Loader;
use external_user_provider::{EupByUserId, EupByUserIdLoaderFn};
use std::collections::HashMap;

type CachedDataLoader<K, V, B> = Loader<K, V, B, HashMap<K, V>>;

pub struct DataLoaders {
    pub e_user_by_user_id: EupByUserId,
}

impl DataLoaders {
    pub fn new(pool: ArcDbPool) -> Self {
        Self {
            e_user_by_user_id: Loader::new(EupByUserIdLoaderFn::new(pool.clone())),
        }
    }
}
