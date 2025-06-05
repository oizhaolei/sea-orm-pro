use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use axum_casbin::CasbinAxumLayer;
use casbin::DefaultModel;
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
        let model = DefaultModel::from_file("config/rbac_model.conf")
            .await
            .unwrap();

        let adapter = SeaOrmAdapter::new(ctx.db.clone()).await.unwrap();
        let mut casbin_middleware = CasbinAxumLayer::new(model, adapter).await.unwrap();
        let enforcer = casbin_middleware.get_enforcer();

        let router = router.layer(Extension(enforcer));
        Ok(router)
    }
}
