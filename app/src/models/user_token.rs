use crate::schema::user_tokens;
use chrono::*;

use crate::models::UserModel as User;
use cuid::cuid;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Identifiable, Associations, Queryable, Clone, Debug)]
#[belongs_to(User)]
#[table_name = "user_tokens"]
pub struct UserTokenModel {
    pub id: String,
    pub token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub refresh_expire_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Identifiable, AsChangeset)]
#[table_name = "user_tokens"]
pub struct NewUserTokenModel<'a> {
    pub id: String,
    pub token: Option<&'a String>,
    pub refresh_token: Option<&'a String>,
    pub user_id: Option<&'a String>,
    pub refresh_expire_at: Option<NaiveDateTime>,
}

const DURATION_OF_REFRESH_TOKEN_IN_MINUTES: i64 = 525600;

impl<'a> Default for NewUserTokenModel<'a> {
    fn default() -> Self {
        Self {
            id: cuid().unwrap(),
            token: None,
            refresh_token: None,
            user_id: None,
            refresh_expire_at: None,
        }
    }
}

impl<'a> NewUserTokenModel<'a> {
    pub fn new(
        token: Option<&'a String>,
        refresh_token: Option<&'a String>,
        user_id: Option<&'a String>,
    ) -> Self {
        let refresh_expire_at = Utc::now()
            .naive_local()
            .checked_add_signed(Duration::minutes(DURATION_OF_REFRESH_TOKEN_IN_MINUTES));
        Self {
            token,
            refresh_token,
            user_id,
            refresh_expire_at,
            ..Default::default()
        }
    }

    pub fn save(self, conn: &MysqlConnection) -> Result<UserTokenModel, Error> {
        use crate::schema::user_tokens::dsl::*;
        diesel::insert_into(user_tokens)
            .values(&self)
            .execute(conn)?;
        user_tokens.filter(id.eq(self.id)).first(conn)
    }
}
