use validator::Validate;
use async_graphql::InputObject;

#[InputObject]
#[derive(Debug, Validate, Deserialize)]
pub struct LocalDataInput {
    #[validate(email(message = "This value should be a E-Mail"))]
    pub email: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[InputObject]
#[derive(Validate)]
pub struct LoginInput {
    #[validate(email(message = "This value should be a E-Mail"))]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[InputObject]
#[derive(Debug, Deserialize, Validate)]
/// User Data to be Updated
pub struct UserInput {
    #[validate(email(message = "This value should be a E-Mail"))]
    /// Changed E-mail
    pub email: Option<String>,
    #[validate(length(min = 6))]
    /// Changed password
    pub password: Option<String>,
}
