pub mod user;

use crate::graphql::guards::*;
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
    pub async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        user::users(ctx)
    }
}
