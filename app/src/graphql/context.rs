use crate::db::mysql::{DbPool, DbPooledConnection};
use crate::graphql::dataloaders::DataLoaders;
use crate::graphql::utils::authorization::AuthorizationService;
use crate::models::{AuthAssignmentModel as AuthAssignment, UserModel as User};
use crate::web_utils::jwt::LoggedUser;
use std::sync::Arc;

pub type ArcDbPool = Arc<DbPool>;
pub type ArcRedisClient = Arc<redis::Client>;

pub struct Context {
    pub pool: ArcDbPool,
    pub redis_client: ArcRedisClient,
    pub dataloaders: DataLoaders,
    pub user: Option<User>,
    pub user_token: Option<String>,
    pub user_assignments: Option<Vec<AuthAssignment>>,
    pub auth_service: AuthorizationService,
}

impl Context {
    pub fn new(user_info: LoggedUser, pool: ArcDbPool, redis_client: ArcRedisClient) -> Context {
        let mut auth_service = AuthorizationService::new(Arc::clone(&pool));
        auth_service
            .init()
            .expect("Error in AuthorizationService Initialization");
        let (user, user_assignments) = {
            let conn: &DbPooledConnection = &pool.get().unwrap();
            let user = User::find_user(&user_info, conn);
            let user_assignments = {
                match &user {
                    Some(user) => user.auth_assignments(conn).ok(),
                    None => None,
                }
            };
            (user, user_assignments)
        };
        Context {
            dataloaders: DataLoaders::new(pool.clone()),
            pool,
            redis_client,
            user,
            user_token: user_info.token,
            user_assignments,
            auth_service,
        }
    }
}
