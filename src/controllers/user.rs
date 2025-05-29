use std::sync::{Arc, Mutex};

use casbin::{Enforcer, RbacApi};
use loco_openapi::prelude::*;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

pub const USERS_TAG: &str = "Users";

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
            name: vec.get(0).cloned().unwrap_or_default(),
            resource: vec.get(1).cloned().unwrap_or_default(),
            action: vec.get(2).cloned().unwrap_or_default(),
        }
    }
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
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    // Give the JWT is valid, return the user profile
    let pid = &auth.claims.pid;
    let name = &auth.claims.pid;
    let email = &auth.claims.pid;

    // my permissions
    let enforcer: Arc<Mutex<Enforcer>> = ctx.shared_store.get().unwrap();
    let e = enforcer.lock().unwrap();

    let permissions: Vec<Permission> = e
        .get_implicit_permissions_for_user(email, None)
        .into_iter()
        .map(Permission::from)
        .collect();

    format::json(CurrentResponse {
        pid: pid.to_string(),
        name: name.to_string(),
        email: email.to_string(),
        permissions,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("user")
        // Fetch user profile
        .add("/current", openapi(get(current), routes!(current)))
}
