use crate::models::user;

use casbin::{CoreApi, DefaultModel, Enforcer, MgmtApi};
use loco_openapi::prelude::*;
use loco_rs::{auth::jwt, hash, prelude::*};
use sea_orm_adapter::SeaOrmAdapter;
use serde::{Deserialize, Serialize};

pub const AUTH_TAG: &str = "Auth";

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PolicyParams {
    pub name: String,
    pub resource: String,
    pub action: String,
}

impl PolicyParams {
    fn to_vec(&self) -> Vec<String> {
        let array = [&self.name, &self.resource, &self.action];
        array.iter().map(|s| s.to_string()).collect()
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

pub async fn create_enforcer(db: DatabaseConnection) -> Enforcer {
    let m = DefaultModel::from_file("config/rbac_model.conf")
        .await
        .unwrap();

    let a = SeaOrmAdapter::new(db).await.unwrap();
    Enforcer::new(m, a).await.unwrap()
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
async fn get_all_policy(State(ctx): State<AppContext>) -> Result<Response> {
    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let all = e.get_all_policy();

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
async fn get_all_grouping_policy(State(ctx): State<AppContext>) -> Result<Response> {
    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let all = e.get_all_grouping_policy();

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
async fn get_all_subjects(State(ctx): State<AppContext>) -> Result<Response> {
    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let all = e.get_all_subjects();

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
async fn get_all_objects(State(ctx): State<AppContext>) -> Result<Response> {
    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let all = e.get_all_objects();

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
async fn get_all_actions(State(ctx): State<AppContext>) -> Result<Response> {
    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let all = e.get_all_actions();

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
async fn get_all_roles(State(ctx): State<AppContext>) -> Result<Response> {
    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let all = e.get_all_roles();

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
async fn add_policy(
    State(ctx): State<AppContext>,
    Json(params): Json<PolicyParams>,
) -> Result<Response> {
    // my permissions
    let mut e = create_enforcer(ctx.db.clone()).await;
    let params = params.to_vec();

    let all = e.add_policy(params).await;

    match all {
        Ok(added) => {
            println!("Policy added: {:?}", added);
            format::json(added)
        }
        Err(_e) => {
            bad_request("Failed to add policy.")
        },
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
}
