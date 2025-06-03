use crate::models::user;

use axum::debug_handler;
use casbin::RbacApi;
use loco_openapi::prelude::*;
use loco_rs::{hash, prelude::*};
use sea_orm::DeleteResult;
use serde::{Deserialize, Serialize};

use super::auth::create_enforcer;

pub const USERS_TAG: &str = "Users";

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListResponse {
    pub data: Vec<user::Model>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Permission {
    pub name: String,
    pub resource: String,
    pub action: String,
}

impl From<Vec<String>> for Permission {
    fn from(vec: Vec<String>) -> Self {
        Permission {
            name: vec.first().cloned().unwrap_or_default(),
            resource: vec.get(1).cloned().unwrap_or_default(),
            action: vec.get(2).cloned().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUserParams {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserParams {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

/// Current
///
/// Current user profile
#[utoipa::path(
    tag = USERS_TAG,
    get,
    path = "/api/user/current",
    security(("jwt_token" = [])),
    responses((status = OK, body = CurrentResponse)),
)]
#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    // Give the JWT is valid, return the user profile
    let email = &auth.claims.pid;
    // Find user by email
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(&ctx.db)
        .await?;
    let Some(user) = user else {
        return unauthorized("unauthorized!");
    };

    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let permissions: Vec<Permission> = e
        .get_implicit_permissions_for_user(&user.email, None)
        .into_iter()
        .map(Permission::from)
        .collect();

    format::json(CurrentResponse {
        pid: user.pid.to_string(),
        name: user.name.to_string(),
        email: user.email.to_string(),
        permissions,
    })
}

// TODO: `getList`      | `GET http://my.api.url/posts?sort=["title","ASC"]&range=[0, 24]&filter={"title":"bar"}` |
// TODO: `getMany`      | `GET http://my.api.url/posts?filter={"ids":[123,456,789]}`                  |
// TODO: `getManyReference` | `GET http://my.api.url/posts?filter={"author_id":345}`                  |
/// List Users
///
/// List Users from the database.
#[utoipa::path(
    get,
    path = "/api/user",
    params(
    ("name" = inline(Option<String>), Query, description="User Name"),
    ("ids" = inline(Option<String>), Query, description="ids"),
    ("page" = inline(Option<usize>), Query, description="Page"),
    ("perPage" = inline(Option<usize>), Query, description="PerPage"),
    ("field" = inline(Option<String>), Query, description="Field"),
    ("order" = inline(Option<String>), Query, description="Order")
    ) ,
    responses((status = OK, body = ListResponse)),
    security(("jwt_token" = [])),
    tag = USERS_TAG
)]
#[debug_handler]
async fn get_list(State(ctx): State<AppContext>) -> Result<Response> {
    let list = user::Entity::find().all(&ctx.db).await?;

    format::json(ListResponse { data: list })
}

// TODO: `getOne`       | `GET http://my.api.url/posts/123`                               |
/// Get single User by id
///
/// Get single user by id from the database
#[utoipa::path(
    get,
    path = "/api/user/{id}",
    params(("id" =i32, Path, description="User Id")),
    responses((status = OK, body = user::Model)),
    security(("jwt_token" = [])),
    tag = USERS_TAG
)]
#[debug_handler]
async fn get_one(
    _auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = user::Entity::find_by_id(id).one(&ctx.db).await?;
    let Some(user) = user else {
        return not_found();
    };

    // my permissions
    let e = create_enforcer(ctx.db.clone()).await;

    let permissions: Vec<Permission> = e
        .get_implicit_permissions_for_user(&user.email, None)
        .into_iter()
        .map(Permission::from)
        .collect();

    format::json(CurrentResponse {
        pid: user.pid.to_string(),
        name: user.name.to_string(),
        email: user.email.to_string(),
        permissions,
    })
}

// TODO: `create`       | `POST http://my.api.url/posts`                              |
/// Create new User
///
/// Create a new User in the database.
#[utoipa::path(
    post,
    path = "/api/user",
    tag = USERS_TAG,
    security(("jwt_token" = [])),
    request_body(content=CreateUserParams, content_type="application/json", description="New User Information"),
    responses(
        (status = 201, description = "User item created successfully", body = user::Model)
    )
)]
#[debug_handler]
async fn create_one(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateUserParams>,
) -> Result<Response> {
    println!("params: {:?}", params);
    let password_hash =
        hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
    let user = user::ActiveModel {
        email: ActiveValue::set(params.email.to_string()),
        password: ActiveValue::set(password_hash),
        name: ActiveValue::set(params.name.to_string()),
        pid: ActiveValue::Set(Uuid::new_v4()),
        api_key: ActiveValue::Set(Uuid::new_v4().to_string()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;

    format::json(user)
}

// TODO: `update`       | `PUT http://my.api.url/posts/123`                               |
/// Update existing User
///
/// Update a User in the database.
#[utoipa::path(
    put,
    path = "/api/user/{id}",
    tag = USERS_TAG,
    security(("jwt_token" = [])),
    params(("id" =i32, Path, description="User Id")),
    request_body(content=UpdateUserParams, content_type="application/json", description="User To Update"),
    responses(
        (status = 200, description = "User item updated successfully", body = user::Model)
    )
)]
#[debug_handler]
async fn update_one(
    _auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateUserParams>,
) -> Result<Response> {
    let user = user::Entity::find_by_id(id).one(&ctx.db).await?;
    let Some(user) = user else {
        return not_found();
    };

    let mut user: user::ActiveModel = user.into();
    if let Some(email) = params.email {
        user.email = Set(email.to_string());
    }
    if let Some(password) = params.password {
        let password_hash =
            hash::hash_password(&password).map_err(|e| ModelError::Any(e.into()))?;
        user.password = Set(password_hash);
    }
    if let Some(name) = params.name {
        user.name = Set(name.to_string());
    }
    let user: user::Model = user.update(&ctx.db).await?;

    format::json(user)
}

// TODO: `updateMany`       | Multiple calls to `PUT http://my.api.url/posts/123`                     |
// TODO: `delete`       | `DELETE http://my.api.url/posts/123`                            |
/// Delete existing User
///
/// Delete a User from the database.
#[utoipa::path(
    delete,
    path = "/api/user/{id}",
    params(("id" = i32, Path, description="User Id")),
    tag = USERS_TAG,
    security(("jwt_token" = [])),
    responses(
        (status = 200, description = "User item deleted successfully", body = String)
    )
)]
#[debug_handler]
async fn delete_one(
    _auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let res: DeleteResult = user::Entity::delete_by_id(id).exec(&ctx.db).await?;

    format::json(res.rows_affected)
}
// TODO: `deleteMany`       | Multiple calls to `DELETE http://my.api.url/posts/123`                  |

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("user")
        .add("", openapi(get(get_list), routes!(get_list)))
        // Fetch user profile
        .add("/current", openapi(get(current), routes!(current)))
        .add("", openapi(post(create_one), routes!(create_one)))
        .add("/{id}", openapi(get(get_one), routes!(get_one)))
        .add("/{id}", openapi(put(update_one), routes!(update_one)))
        .add("/{id}", openapi(delete(delete_one), routes!(delete_one)))
}
