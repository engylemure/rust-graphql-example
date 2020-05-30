use crate::errors::{SrvError, UnauthorizedInfo};
use crate::graphql::context::Context;
use async_graphql::{guard::Guard, Context as GqlContext, FieldResult};

#[derive(Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug)]
pub struct RoleGuard {
    pub role: Role,
}

#[derive(Debug)]
pub struct AuthGuard {}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &GqlContext<'_>) -> FieldResult<()> {
        let context = ctx.data::<Context>();
        let auth_service = &context.auth_service;
        let is_authorized = match &self.role {
            Role::User => auth_service.is_user(&context.user_assignments),
            Role::Admin => auth_service.is_admin(&context.user_assignments),
        };
        dbg!(&self);
        dbg!(is_authorized);
        dbg!(&context.user_assignments);
        if is_authorized
        {
            Ok(())
        } else {
            Err(SrvError::Unauthorized(UnauthorizedInfo {
                data: String::from("You are not Authorized to acess This!"),
            })
            .into())
        }
    }
}

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &GqlContext<'_>) -> FieldResult<()> {
        let context = ctx.data::<Context>();
        if context.user.is_some() {
            Ok(())
        } else {
            Err(SrvError::Unauthorized(UnauthorizedInfo {
                data: String::from("You are not Authenticated!"),
            })
            .into())
        }
    }
}
