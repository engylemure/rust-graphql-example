use crate::graphql::objects::external_user_provider::ExternalUserProvider;
use crate::graphql::objects::user::User;
use async_graphql::*;

#[Interface(field(name = "id", type = "ID"))]
pub enum Node {
    User(User),
    ExternalUserProvider(ExternalUserProvider),
}
