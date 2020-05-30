use async_graphql::{Context as GqlContext, FieldError};
use diesel::prelude::*;
use chrono::Utc;
use validator::Validate;
use crate::graphql::context::Context;
use crate::graphql::input::{LocalDataInput, LoginInput, UserInput};
use crate::graphql::utils::authorization::assert_user;
use crate::utils::argon::{make_hash};
use crate::models::{UserModel as User, NewUser, NewAuthAssignmentModel as NewAuthAssignment, UserTokenModel as UserToken, UpdatedUserModel as UpdatedUser};
use crate::{errors::{SrvError, UnauthorizedInfo}, graphql::objects::user::Token, };

pub type AuthResult = Result<Token, SrvError>;

pub fn register(ctx: &GqlContext<'_>, input: LocalDataInput) -> AuthResult {
    let context = ctx.data::<Context>();
    input.validate()?;
    let conn: &MysqlConnection = &context.pool.get().unwrap();
    let LocalDataInput {
        email, password, ..
    } = input;
    let user = NewUser::new(&email, &password).save(conn)?;
    NewAuthAssignment::new("user", &user.id).save(conn)?;
    Ok(Token::from_user(user)?.save(conn)?)
}

pub fn login(ctx: &GqlContext<'_>, input: LoginInput) -> AuthResult {
    use crate::schema::users::dsl::*;
    let context = ctx.data::<Context>();
    input.validate()?;
    let conn: &MysqlConnection = &context.pool.get().unwrap();
    if let Ok(user) = users.filter(email.eq(input.email)).first::<User>(conn) {
        return if make_hash(&input.password, &user.salt) == user.hash {
            conn.transaction::<_, SrvError, _>(|| Token::from_user(user)?.save(conn))
        } else {
            Err(SrvError::Unauthorized(UnauthorizedInfo {
                data: String::from("Wrong Password!"),
            }))
        };
    } else {
        Err(SrvError::Unauthorized(UnauthorizedInfo {
            data: String::from("Wrong Email!"),
        }))
    }
}

pub fn refresh_token(ctx: &GqlContext<'_>, refresh_token: String) -> AuthResult {
    use crate::schema::user_tokens::dsl::{refresh_expire_at, refresh_token as r_token, user_tokens};
    use crate::schema::users::dsl::{id, users};
    let context = ctx.data::<Context>();
    let conn: &MysqlConnection = &context.pool.get().unwrap();
    conn.transaction::<_, SrvError, _>(|| {
        let user_token_result = user_tokens
            .filter(r_token.eq(refresh_token))
            .filter(refresh_expire_at.ge(Utc::now().naive_local()))
            .first::<UserToken>(conn);
        match user_token_result {
            Ok(user_token) => {
                diesel::delete(&user_token).execute(conn)?;
                let user = users
                    .filter(id.eq(user_token.user_id))
                    .first::<User>(conn)?;
                Token::from_user(user)?.save(conn)
            }
            Err(_) => Err(SrvError::Unauthorized(UnauthorizedInfo {
                data: String::from("Invalid Refresh Token!"),
            })),
        }
    })
}

pub fn logout(ctx: &GqlContext<'_>,) -> Result<bool, SrvError> {
    use crate::schema::user_tokens::dsl::{token, user_tokens};
    let context = ctx.data::<Context>();
    assert_user(&context.user)?;
    let conn: &MysqlConnection = &context.pool.get().unwrap();
    conn.transaction::<_, SrvError, _>(|| {
        let user_token = context.user_token.as_ref().unwrap();
        Ok(diesel::delete(user_tokens.filter(token.eq(user_token))).execute(conn)? > 0)
    })
}

pub fn update_user(ctx: &GqlContext<'_>, input: UserInput) -> AuthResult {
    use crate::schema::user_tokens::dsl::*;
    let context = ctx.data::<Context>();
    let user = assert_user(&context.user)?;
    return match input.validate() {
        Ok(_) => {
            let conn: &MysqlConnection = &context.pool.get().unwrap();
            conn.transaction::<_, SrvError, _>(|| {
                let updated_user =
                    UpdatedUser::new(input.email, input.password).update(user, conn)?;
                diesel::delete(user_tokens.filter(user_id.eq(&user.id))).execute(conn)?;
                Token::from_user(updated_user)?.save(conn)
            })
        }
        Err(e) => Err(SrvError::ValidationError(e.into())),
    };
}
