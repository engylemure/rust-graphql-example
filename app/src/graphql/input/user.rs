use crate::graphql::objects::external_user_provider::UserProvider;
use async_graphql::InputObject;
use validator::Validate;

#[InputObject]
#[derive(Debug, Validate, Deserialize)]
pub struct UserRegisterInput {
    #[validate(email(message = "This value should be a E-Mail"))]
    pub email: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[InputObject]
#[derive(Debug, Validate, Deserialize)]
pub struct UserExternalDataInput {
    #[field(desc("Token provided by the UserProvider"))]
    pub token: String,
    pub provider: UserProvider,
}

#[InputObject]
#[derive(Validate)]
pub struct UserLoginInput {
    #[validate(email(message = "This value should be a E-Mail"))]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[InputObject]
#[derive(Debug, Deserialize, Validate)]
/// User Data to be Updated
pub struct UserUpdateInput {
    #[validate(email(message = "This value should be a E-Mail"))]
    /// Changed E-mail
    pub email: Option<String>,
    #[validate(length(min = 6))]
    /// Changed password
    pub password: Option<String>,
}
