use crate::models::UserModel as User;
use crate::schema::auth_assignments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::mysql::MysqlConnection;

#[derive(Associations, Queryable, Clone, Debug)]
#[belongs_to(User)]
#[table_name = "auth_assignments"]
pub struct AuthAssignmentModel {
    pub item_name: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "auth_assignments"]
pub struct NewAuthAssignmentModel<'a> {
    item_name: &'a str,
    user_id: &'a str,
}

impl<'a> NewAuthAssignmentModel<'a> {
    pub fn new(item_name: &'a str, user_id: &'a str) -> NewAuthAssignmentModel<'a> {
        NewAuthAssignmentModel { item_name, user_id }
    }
    pub fn save(&self, conn: &MysqlConnection) -> Result<AuthAssignmentModel, Error> {
        use crate::schema::auth_assignments::dsl::*;
        diesel::insert_into(auth_assignments)
            .values(self)
            .execute(conn)?;
        auth_assignments.order(created_at.desc()).first(conn)
    }
}
