use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::Router as AxumRouter;
use casbin::{CoreApi, DefaultModel, Enforcer};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};
use sea_orm_adapter::SeaOrmAdapter;

pub struct CasbinEnforcerInitializer;

#[async_trait]
impl Initializer for CasbinEnforcerInitializer {
    fn name(&self) -> String {
        "casbin-enforcer".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let m = DefaultModel::from_file("config/rbac_model.conf")
            .await
            .unwrap();

        let a = SeaOrmAdapter::new(ctx.db.clone()).await.unwrap();
        let e = Enforcer::new(m, a).await.unwrap();

        let state = Arc::new(Mutex::new(e));
        ctx.shared_store.insert(state);
        Ok(router)
    }
}
