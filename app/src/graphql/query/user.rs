use async_graphql::Context as GqlContext;
use diesel::prelude::*;
use crate::graphql::context::Context;
use crate::models::UserModel as User;

pub fn me(ctx: &GqlContext) -> Option<User> {
    let ctx = ctx.data::<Context>();
    ctx.user.clone()
}

pub fn users(ctx: &GqlContext) -> Vec<User> {
    use crate::schema::users::dsl::*;
    let context = ctx.data::<Context>();
    let conn: &MysqlConnection = &context.pool.get().unwrap();
    let users_data = users.load::<User>(conn).expect("");
    users_data
}