use async_graphql::{Enum, ID};
use chrono::*;

use crate::graphql::context::Context;
use crate::models::{ExternalUserProviderModel as ExternalUserProvider, UserProvider as UProvider};

#[async_graphql::Object(desc = "A user provider")]
impl ExternalUserProvider {
    pub async fn id(&self) -> &String {
        &self.id
    }
    pub async fn email(&self) -> &Option<String> {
        &self.email
    }
    pub async fn provider(&self) -> UserProvider {
        self.provider.into()
    }
    pub async fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }
    pub async fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }
    pub async fn deleted(&self) -> bool {
        self.deleted
    }
}

#[Enum(desc = "A External Authentication Provider")]
#[derive(Debug, Copy, Deserialize, PartialEq, Clone)]
pub enum UserProvider {
    Facebook,
    Google,
    Apple,
}

impl From<UProvider> for UserProvider {
    fn from(up: UProvider) -> Self {
        match up {
            UProvider::Apple => UserProvider::Apple,
            UProvider::Facebook => UserProvider::Facebook,
            UProvider::Google => UserProvider::Google,
        }
    }
}
