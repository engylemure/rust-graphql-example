use async_graphql::{guard::Guard, Context as GqlContext, FieldResult};
use crate::errors::{SrvError, UnauthorizedInfo};
use crate::graphql::context::Context;

#[derive(Debug)]
pub enum Role {
    Admin,
    User
}

#[derive(Debug)]
pub struct RoleGuard {
    pub role: Role
}

#[derive(Debug)]
pub struct AuthGuard {}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &GqlContext<'_>) -> FieldResult<()> {
        let context = ctx.data::<Context>();
        dbg!(&self);
        dbg!(&context.user);
        let role = String::from(match &self.role {
            Role::Admin => "admin",
            Role::User => "user"
        });
        dbg!(&role);
        if context.auth_service.is_authorized(&context.user_assignments, role) {
            Ok(())
        } else {
            Err(SrvError::Unauthorized(UnauthorizedInfo {
                data: String::from("You has no permission to access This!"),
            }).into())
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
                data: String::from("You are not authenticated!"),
            }).into())
        }
    }
}