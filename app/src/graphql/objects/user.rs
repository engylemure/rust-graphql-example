use async_graphql::{Context, FieldResult, ID};
use chrono::*;
use diesel::mysql::MysqlConnection;
use uuid::Uuid;
use crate::models::{UserModel as User, NewUserTokenModel as NewUserToken, UpdatedUserModel as UpdatedUser};
use crate::models::external_user_provider::ExternalUserProviderModel as ExternalUserProvider;
use crate::{web_utils::jwt::create_token, errors::SrvError};

#[async_graphql::Object(desc = "A user")]
impl User {
    pub async fn id(&self) -> &String {
        &self.id
    }
    pub async fn email(&self) -> &String {
        &self.email
    }
    pub async fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }
    pub async fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)    
    }
    async fn providers(&self, _context: &Context<'_> ) -> FieldResult<Vec<ExternalUserProvider>> {
        Ok(Vec::new())
    }
}


/// Token Object with the Auth Token Value a Refresh Token and the User associated with
pub struct Token {
    pub value: String,
    pub refresh_token: String,
    pub user: User,
}

#[async_graphql::Object(desc = "The token object with user information",)]
impl Token {
    /// Value of this token
    pub async fn value(&self) -> &String {
        &self.value
    }
    /// Refresh Token it is a token that can be reused for getting a new Token
    pub async fn refresh_token(&self) -> &String {
        &self.refresh_token
    }
    /// User associated to this Token
    pub async fn user(&self) -> &User {
        &self.user
    }
}

impl Token {
    pub fn from_user(user: User) -> Result<Token, SrvError> {
        match create_token(&user.id) {
            Some(value) => Ok(Token {
                value,
                refresh_token: Uuid::new_v4().to_string(),
                user,
            }),
            None => Err(SrvError::InternalServerError),
        }
    }

    pub fn save(self, conn: &MysqlConnection) -> Result<Token, SrvError> {
        NewUserToken::new(
            Some(&self.value),
            Some(&self.refresh_token),
            Some(&self.user.id),
        )
        .save(conn)?;
        Ok(self)
    }
}


