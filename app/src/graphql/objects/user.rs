use crate::graphql::context::Context as Ctx;
use crate::models::external_user_provider::ExternalUserProviderModel as ExternalUserProvider;
use crate::models::{
    NewUserTokenModel as NewUserToken, UpdatedUserModel as UpdatedUser, UserModel as User,
};
use crate::{errors::SrvError, web_utils::jwt::create_token};
use async_graphql::connection::{Connection as GqlConn, DataSource, Edge, EmptyFields};
use async_graphql::{Context, DataSource, FieldError, FieldResult, ID};
use chrono::*;
use diesel::prelude::*;
use uuid::Uuid;

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
    async fn providers(&self, _ctx: &Context<'_>) -> FieldResult<Vec<ExternalUserProvider>> {
        Ok(Vec::new())
    }
}

/// Token Object with the Auth Token Value a Refresh Token and the User associated with
pub struct Token {
    pub value: String,
    pub refresh_token: String,
    pub user: User,
}

#[async_graphql::Object(desc = "The token object with user information")]
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

pub struct UserConnection;

const MAX_PAGE_SIZE: i64 = 100;

pub type UserConnResult = FieldResult<GqlConn<ID, User, EmptyFields, EmptyFields>>;

#[DataSource]
impl DataSource for UserConnection {
    type CursorType = ID;
    type NodeType = User;
    // We don't need to extend the connection fields, so this can be empty
    type ConnectionFieldsType = EmptyFields;

    // We don't need to extend the edge fields, so this can be empty
    type EdgeFieldsType = EmptyFields;

    async fn execute_query(
        &self,
        ctx: &Context<'_>,
        after: Option<ID>,
        before: Option<ID>,
        first: Option<usize>,
        last: Option<usize>,
    ) -> UserConnResult {
        use crate::models::utils::pagination::{Paginate, PaginatedData};
        use crate::schema::users::{all_columns, id, table};
        let context = ctx.data::<Ctx>();
        let conn: &MysqlConnection = &context.pool.get().unwrap();
        let PaginatedData {
            data,
            total_pages,
            page,
            ..
        }: PaginatedData<User> = match (after, before, first, last) {
            (Some(after), Some(before), Some(first), None) => table
                .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
                .paginate(1)
                .per_page(MAX_PAGE_SIZE)
                .load_and_count_pages(conn)?,
            // (Some(after), Some(before), None, Some(last)) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            //  },
            // (Some(after), Some(before), None, None) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            // },
            // (None, Some(before), Some(first), None) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            // },
            // (None, Some(before), None, Some(last)) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            // },
            // (None, Some(before), None, None) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            // },
            // (Some(after), None, Some(first), None) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            //  },
            // (Some(after), None, None, Some(last)) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            //  },
            // (Some(after), None, None, None) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            //  },
            (None, None, Some(first), None) => table
                .select(all_columns)
                .paginate(1)
                .per_page(first as i64)
                .load_and_count_pages(conn)?,
            // (None, None, None, Some(last)) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            //  },
            // (None, None, None, None) => {
            //     table
            //     .filter(id.ge(after.as_str()).and(id.le(before.as_str())))
            //     .paginate(1)
            //     .per_page(MAX_PAGE_SIZE)
            //     .load_and_count_pages(conn)?
            // },
            _ => return Err(FieldError(String::from("Connection error"), None)),
        };
        let mut connection = GqlConn::new(page > 1, page < total_pages);
        connection.append(data.into_iter().map(|user_model| {
            Edge::with_additional_fields(ID::from(user_model.id.clone()), user_model, EmptyFields)
        }));
        Ok(connection)
    }
}
