use crate::models::UserModel as User;
use crate::schema::external_user_providers;
use chrono::{NaiveDateTime, Utc};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Identifiable, Associations, Queryable, Clone)]
#[belongs_to(User)]
#[table_name = "external_user_providers"]
pub struct ExternalUserProviderModel {
    pub id: String,
    pub user_id: String,
    pub external_id: String,
    pub email: Option<String>,
    pub provider: UserProvider,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

#[DieselType = "UserProviderMapping"]
#[derive(Debug, PartialEq, DbEnum, Deserialize, Copy, Clone)]
pub enum UserProvider {
    Facebook,
    Google,
    Apple,
}

#[derive(Insertable, Debug, Clone, PartialEq)]
#[table_name = "external_user_providers"]
pub struct NewExternalUserProviderModel<'a> {
    pub id: String,
    pub user_id: String,
    pub external_id: String,
    pub provider: UserProvider,
    pub email: Option<&'a String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted: Option<bool>,
}

impl<'a> NewExternalUserProviderModel<'a> {
    pub fn new(
        user_id: String,
        external_id: String,
        provider: UserProvider,
        email: &'a String,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: cuid::cuid().unwrap(),
            user_id,
            external_id,
            provider,
            email: Some(email),
            created_at: Some(now.clone()),
            updated_at: Some(now),
            deleted: Some(false),
        }
    }

    pub fn save(self, conn: &MysqlConnection) -> Result<ExternalUserProviderModel, Error> {
        use crate::schema::external_user_providers::dsl::*;
        diesel::insert_into(external_user_providers)
            .values(&self)
            .execute(conn)?;
        external_user_providers.filter(id.eq(self.id)).first(conn)
    }
}
