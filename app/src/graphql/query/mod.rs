pub mod user;

use crate::graphql::{guards::*, objects::user::UserConnResult};
use async_graphql::guard::Guard;
use async_graphql::*;

use crate::models::UserModel as User;
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    #[field(guard(AuthGuard()))]
    pub async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        user::me(ctx)
    }

    #[field(guard(RoleGuard(role = "Role::Admin")))]
    pub async fn users(
        &self,
        ctx: &Context<'_>,
        after: Option<ID>,
        before: Option<ID>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> UserConnResult {
        user::users(ctx, after, before, first, last).await
    }
}
