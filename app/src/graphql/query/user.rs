use crate::graphql::{
    context::Context,
    objects::user::{UserConnResult, UserConnection},
};
use crate::models::UserModel as User;
use async_graphql::{connection::DataSource, Context as GqlContext, ID};
use diesel::prelude::*;

pub fn me(ctx: &GqlContext) -> Option<User> {
    let ctx = ctx.data::<Context>();
    ctx.user.clone()
}

pub async fn users(
    ctx: &GqlContext<'_>,
    after: Option<ID>,
    before: Option<ID>,
    first: Option<i32>,
    last: Option<i32>,
) -> UserConnResult {
    UserConnection
        .query(
            ctx,
            after.map(|val| val.to_string()),
            before.map(|val| val.to_string()),
            first,
            last,
        )
        .await
}
