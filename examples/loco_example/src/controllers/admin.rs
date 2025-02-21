use loco_rs::prelude::*;
use sea_orm_pro::ConfigParser;

pub async fn config(State(_ctx): State<AppContext>) -> Result<Response> {
    let config = ConfigParser::new()
        .load_config("pro_admin")
        .expect("Invalid TOML Config");
    format::json(config)
}

pub fn routes() -> Routes {
    Routes::new().prefix("admin").add("/config", get(config))
}
