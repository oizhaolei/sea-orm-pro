use std::path::Path;
use loco_openapi::prelude::*;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;

use crate::{controllers, initializers::casbin_enforcer::CasbinEnforcerInitializer, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(loco_openapi::OpenapiInitializerWithSetup::new(
                |ctx| {
                    #[derive(OpenApi)]
                    #[openapi(
                    modifiers(&SecurityAddon),
                    info(
                        title = "SeaORM Pro",
                        description = "Some REST API for SeaORM Pro."
                    )
                )]
                    struct ApiDoc;
                    set_jwt_location(ctx.into());

                    ApiDoc::openapi()
                },
                // When using automatic schema collection only
                None,
                // When using manual schema collection
                // Manual schema collection can also be used at the same time as automatic schema collection
                // Some(vec![controllers::album::api_routes()]),
            )),
            Box::new(CasbinEnforcerInitializer),
        ])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        // Register all routes
        AppRoutes::with_default_routes()
            .prefix("/api")
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
            .add_route(controllers::graphql::routes())
            .add_route(controllers::admin::routes())
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(tasks: &mut Tasks) {
        // Register all tasks
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(_ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    async fn seed(_ctx: &AppContext, _base: &Path) -> Result<()> {
        Ok(())
    }
}
