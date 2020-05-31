use crate::graphql::guards::*;
use crate::graphql::input::*;
use crate::graphql::objects::user::Token;
use async_graphql::{guard::Guard, Context, FieldError};

pub mod user;
pub struct Mutation;

type AuthResult = Result<Token, FieldError>;

#[async_graphql::Object]
impl Mutation {
    pub async fn login(&self, ctx: &Context<'_>, input: UserLoginInput) -> AuthResult {
        Ok(user::login(ctx, input)?)
    }

    pub async fn login_with_external_user(
        &self,
        ctx: &Context<'_>,
        input: UserExternalDataInput,
    ) -> AuthResult {
        Ok(user::login_with_external_user(ctx, input)?)
    }

    pub async fn register(&self, ctx: &Context<'_>, user: UserRegisterInput) -> AuthResult {
        Ok(user::register(ctx, user)?)
    }

    pub async fn refresh_token(&self, ctx: &Context<'_>, refresh_token: String) -> AuthResult {
        Ok(user::refresh_token(ctx, refresh_token)?)
    }

    #[field(guard(AuthGuard()))]
    pub async fn logout(&self, ctx: &Context<'_>) -> Result<bool, FieldError> {
        Ok(user::logout(ctx)?)
    }

    #[field(guard(AuthGuard()))]
    pub async fn update_user(&self, ctx: &Context<'_>, input: UserUpdateInput) -> AuthResult {
        Ok(user::update_user(ctx, input)?)
    }
}
