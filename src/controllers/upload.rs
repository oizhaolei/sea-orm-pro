use std::path::PathBuf;

use axum::extract::Multipart;
use loco_openapi::prelude::openapi;
use loco_openapi::prelude::*;
use loco_rs::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, ToSchema)]
#[allow(unused)]
struct HelloForm {
    name: String,
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    file: String,
}
/// File upload example
///
/// ## Request Example
///
/// curl -H "Content-Type: multipart/form-data" -F "file=@./README.md" http://localhost:8086/api/upload/file
#[utoipa::path(
    post,
    path = "/api/upload/file",
    security(("jwt_token" = [])),
    request_body(content = HelloForm, content_type = "multipart/form-data"),
    responses((status = OK, body = String)),
)]
async fn upload_file(State(ctx): State<AppContext>, mut multipart: Multipart) -> Result<Response> {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err,"could not readd multipart");
        Error::BadRequest("could not readd multipart".into())
    })? {
        let file_name = match field.file_name() {
            Some(file_name) => file_name.to_string(),
            _ => return Err(Error::BadRequest("file name not found".into())),
        };

        let content = field.bytes().await.map_err(|err| {
            tracing::error!(error = ?err,"could not readd bytes");
            Error::BadRequest("could not readd bytes".into())
        })?;

        let path = PathBuf::from("folder").join(file_name);
        ctx.storage
            .as_ref()
            .upload(path.as_path(), &content)
            .await?;

        file = Some(path);
    }

    file.map_or_else(not_found, |path| format::json(path.as_path()))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("upload")
        .add("/file", openapi(post(upload_file), routes!(upload_file)))
}
