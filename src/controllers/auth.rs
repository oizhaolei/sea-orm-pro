use std::sync::Arc;

use crate::models::user;

use axum::extract::Extension;
use casbin::{CachedEnforcer, MgmtApi};
use loco_openapi::prelude::*;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub const AUTH_TAG: &str = "Auth";

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PolicyParams {
    pub subject: String,
    pub object: String,
    pub action: String,
}

impl PolicyParams {
    fn to_vec(&self) -> Vec<String> {
        let array = [&self.subject, &self.object, &self.action];
        array.iter().map(|s| s.to_string()).collect()
    }
}

impl From<Vec<String>> for PolicyParams {
    fn from(vec: Vec<String>) -> Self {
        PolicyParams {
            subject: vec.first().cloned().unwrap_or_default(),
            object: vec.get(1).cloned().unwrap_or_default(),
            action: vec.get(2).cloned().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    pub fn new(user: &user::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: true,
        }
    }
}

/// Login
///
/// Try to login with email and password.
#[utoipa::path(
        tag = AUTH_TAG,
        post,
        path = "/api/auth/login",
        request_body(content=LoginParams, content_type="application/json", description="login"),
        responses(
            (status = 200, description = "User login successfully", body = LoginResponse)
        )
)]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    // Find user by email
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&params.email))
        .one(&ctx.db)
        .await?;
    let Some(user) = user else {
        return unauthorized("unauthorized!");
    };

    // Verify password
    if !hash::verify_password(&params.password, &user.password) {
        return unauthorized("unauthorized!");
    }

    // Generate the JWT
    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = jwt::JWT::new(&jwt_secret.secret)
        .generate_token(
            jwt_secret.expiration,
            params.email.to_string(),
            serde_json::Map::<String, serde_json::Value>::new(),
        )
        .unwrap();

    // Login success
    format::json(LoginResponse::new(&user, &token))
}

/// get_all_policy
#[utoipa::path(
    get,
    path = "/api/auth/get_all_policy",
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
async fn get_all_policy(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
) -> Result<Response> {
    // my permissions

    let lock = enforcer.write().await;
    let all = lock.get_all_policy();
    drop(lock);

    format::json(all)
}

/// get_all_grouping_policy
#[utoipa::path(
    get,
    path = "/api/auth/get_all_grouping_policy",
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
async fn get_all_grouping_policy(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
) -> Result<Response> {
    // my permissions

    let lock = enforcer.write().await;
    let all = lock.get_all_grouping_policy();
    drop(lock);

    format::json(all)
}

/// get_all_subjects
#[utoipa::path(
    get,
    path = "/api/auth/get_all_subjects",
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
async fn get_all_subjects(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
) -> Result<Response> {
    // my permissions

    let lock = enforcer.write().await;
    let all = lock.get_all_subjects();
    drop(lock);

    format::json(all)
}

/// get_all_objects
#[utoipa::path(
    get,
    path = "/api/auth/get_all_objects",
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
async fn get_all_objects(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
) -> Result<Response> {
    // my permissions

    let lock = enforcer.write().await;
    let all = lock.get_all_objects();
    drop(lock);

    format::json(all)
}

/// get_all_actions
#[utoipa::path(
    get,
    path = "/api/auth/get_all_actions",
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
async fn get_all_actions(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
) -> Result<Response> {
    // my permissions

    let lock = enforcer.write().await;
    let all = lock.get_all_actions();
    drop(lock);

    format::json(all)
}

/// get_all_roles
#[utoipa::path(
    get,
    path = "/api/auth/get_all_roles",
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
async fn get_all_roles(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
) -> Result<Response> {
    // my permissions

    let lock = enforcer.write().await;
    let all = lock.get_all_roles();
    drop(lock);

    format::json(all)
}

/// add_policy
#[utoipa::path(
    post,
    path = "/api/auth/add_policy",
    request_body(content=PolicyParams, content_type="application/json", description=""),
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
pub async fn add_policy(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
    Json(params): Json<PolicyParams>,
) -> Result<Response> {
    // my permissions
    let params = params.to_vec();

    let mut lock = enforcer.write().await;
    let all = lock.add_policy(params).await;
    drop(lock);

    match all {
        Ok(added) => {
            println!("Policy added: {:?}", added);
            format::json(added)
        }
        Err(_e) => bad_request("Failed to add policy."),
    }
}

/// remove_policy
#[utoipa::path(
    post,
    path = "/api/auth/remove_policy",
    request_body(content=PolicyParams, content_type="application/json", description=""),
    responses((status = OK, body = String)),
    security(("jwt_token" = [])),
    tag = AUTH_TAG
)]
async fn remove_policy(
    State(_ctx): State<AppContext>,
    Extension(enforcer): Extension<Arc<RwLock<CachedEnforcer>>>,
    Json(params): Json<PolicyParams>,
) -> Result<Response> {
    // my permissions
    let params = params.to_vec();

    let mut lock = enforcer.write().await;
    let all = lock.remove_policy(params).await;
    drop(lock);

    match all {
        Ok(removed) => {
            println!("Policy removeed: {:?}", removed);
            format::json(removed)
        }
        Err(_e) => bad_request("Failed to remove policy."),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        // Authentication route prefix
        .prefix("auth")
        // Handling login with password
        .add("/login", openapi(post(login), routes!(login)))
        .add(
            "/get_all_policy",
            openapi(get(get_all_policy), routes!(get_all_policy)),
        )
        .add(
            "/get_all_grouping_policy",
            openapi(
                get(get_all_grouping_policy),
                routes!(get_all_grouping_policy),
            ),
        )
        .add(
            "/get_all_subjects",
            openapi(get(get_all_subjects), routes!(get_all_subjects)),
        )
        .add(
            "/get_all_objects",
            openapi(get(get_all_objects), routes!(get_all_objects)),
        )
        .add(
            "/get_all_actions",
            openapi(get(get_all_actions), routes!(get_all_actions)),
        )
        .add(
            "/get_all_roles",
            openapi(get(get_all_roles), routes!(get_all_roles)),
        )
        .add(
            "/add_policy",
            openapi(post(add_policy), routes!(add_policy)),
        )
        .add(
            "/remove_policy",
            openapi(post(remove_policy), routes!(remove_policy)),
        )
}
