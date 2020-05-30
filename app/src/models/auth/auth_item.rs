use chrono::NaiveDateTime;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Associations, Queryable, Clone, Debug)]
pub struct AuthItemModel {
    pub name: String,
    pub r#type: i16,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AuthItemModel {
    pub fn children(&self, conn: &MysqlConnection) -> Result<Vec<AuthItemModel>, Error> {
        use crate::schema::auth_item_children::dsl::{auth_item_children, child, parent};
        use crate::schema::auth_items::{
            all_columns,
            dsl::{auth_items, name},
        };
        auth_items
            .inner_join(auth_item_children.on(child.eq(name)))
            .filter(parent.eq(self.name.clone()))
            .select(all_columns)
            .load(conn)
    }
}
