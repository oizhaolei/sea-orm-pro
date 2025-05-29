use loco_openapi::prelude::*;
use loco_rs::prelude::*;

pub const USERS_TAG: &str = "Users";

/// Current
///
/// Current user profile
#[utoipa::path(
    tag = USERS_TAG,
    get,
    path = "/api/user/current",
    security(("jwt_token" = [])),
    responses((status = OK, body = String)),
)]
async fn current(auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    // Give the JWT is valid, return the user profile
    format::json(serde_json::json!({
        "pid": auth.claims.pid,
        "name": auth.claims.pid,
        "email": auth.claims.pid,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("user")
        // Fetch user profile
        .add("/current", openapi(get(current), routes!(current)))
}
