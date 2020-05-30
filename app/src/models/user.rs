use crate::models::{AuthAssignmentModel, UserTokenModel};
use crate::schema::users;
use crate::utils::argon::{make_hash, make_salt};
use chrono::{NaiveDateTime, Utc};
use cuid::cuid;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Identifiable, Associations, Queryable, Clone, Debug)]
#[table_name = "users"]
pub struct UserModel {
    pub id: String,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

impl UserModel {
    pub fn auth_assignments(
        &self,
        conn: &MysqlConnection,
    ) -> Result<Vec<AuthAssignmentModel>, Error> {
        use crate::schema::auth_assignments::dsl::*;
        auth_assignments
            .filter(user_id.eq(self.id.to_string()))
            .load(conn)
    }
    pub fn find_user(user_info: &SlimUser, conn: &MysqlConnection) -> Option<UserModel> {
        use crate::schema::user_tokens::dsl::{token, user_id, user_tokens};
        use crate::schema::users::dsl::{id, users};
        match users
            .inner_join(user_tokens.on(user_id.eq(id)))
            .filter(id.eq(user_info.id.as_ref()?))
            .filter(token.eq(user_info.token.as_ref()?))
            .first::<(UserModel, UserTokenModel)>(conn)
        {
            Ok((user, ..)) => Some(user),
            Err(_) => None,
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub salt: Option<String>,
    pub email: Option<&'a String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted: Option<bool>,
}

impl<'a> Default for NewUser<'a> {
    fn default() -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Some(cuid().unwrap()),
            hash: None,
            salt: None,
            email: None,
            created_at: Some(now.clone()),
            updated_at: Some(now),
            deleted: Some(false),
        }
    }
}

impl<'a> NewUser<'a> {
    pub fn new_without_pw(email: &'a String) -> NewUser<'a> {
        NewUser {
            email: Some(email),
            ..Self::default()
        }
    }

    pub fn new(email: &'a String, password: &'a String) -> NewUser<'a> {
        let salt = make_salt();
        let hash = make_hash(password, &salt);
        NewUser {
            salt: Some(salt),
            hash: Some(hash),
            ..Self::new_without_pw(email)
        }
    }

    pub fn save(self, conn: &MysqlConnection) -> Result<UserModel, Error> {
        use crate::schema::users::dsl::*;
        diesel::insert_into(users).values(&self).execute(conn)?;
        users.filter(id.eq(self.id.unwrap())).first(conn)
    }
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UpdatedUserModel {
    pub email: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub salt: Option<String>,
}

impl UpdatedUserModel {
    pub fn new(email: Option<String>, password: Option<String>) -> UpdatedUserModel {
        let (salt, hash) = {
            if let Some(password) = password {
                let salt = make_salt();
                let hash = make_hash(&password, &salt);
                (Some(salt), Some(hash))
            } else {
                (None, None)
            }
        };
        Self { email, hash, salt }
    }

    pub fn update(&self, user: &UserModel, conn: &MysqlConnection) -> Result<UserModel, Error> {
        use crate::schema::users::dsl::*;
        diesel::update(users)
            .filter(id.eq(&user.id))
            .set(self)
            .execute(conn)?;
        users.filter(id.eq(&user.id)).first(conn)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SlimUser {
    pub id: Option<String>,
    pub token: Option<String>,
}

impl From<UserModel> for SlimUser {
    fn from(user: UserModel) -> Self {
        SlimUser {
            id: Some(user.id),
            token: None,
        }
    }
}
