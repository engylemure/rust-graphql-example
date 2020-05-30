use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GQLResponse};
use async_graphql::Data;
use async_graphql_actix_web::{GQLRequest, WSSubscription};

use crate::db::mysql::DbPool;
use crate::graphql::{context::Context, Schema};
use crate::models::SlimUser;
use crate::web_utils::jwt::{decode_token, token_from_value};

pub async fn gql(
    schema: web::Data<Schema>,
    user: SlimUser,
    mysql: web::Data<DbPool>,
    redis: web::Data<redis::Client>,
    gql_request: GQLRequest,
) -> web::Json<GQLResponse> {
    let pool = mysql.into_inner();
    let redis_client = redis.into_inner();
    let ctx = Context::new(user, pool, redis_client);
    let req = gql_request.into_inner().data(ctx);
    web::Json(GQLResponse(req.execute(&schema).await))
}

pub async fn gql_subscriptions(
    schema: web::Data<Schema>,
    req: HttpRequest,
    mysql: web::Data<DbPool>,
    redis: web::Data<redis::Client>,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let actor = WSSubscription::new(&schema);
    let pool = mysql.into_inner();
    let redis_client = redis.into_inner();
    let actor = actor.init_context_data(move |payload| {
        let token = token_from_value(&payload);
        let user = token
            .and_then(|token| decode_token(&token))
            .unwrap_or(SlimUser::default());
        let mut data = Data::default();
        let ctx = Context::new(user, pool.clone(), redis_client.clone());
        data.insert(ctx);
        Ok(data)
    });
    ws::start_with_protocols(actor, &["graphql-ws"], &req, payload)
}

pub async fn gql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}
