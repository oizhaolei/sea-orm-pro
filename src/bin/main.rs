use loco_rs::cli;
use migration::Migrator;
use sea_orm_pro_backend::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load `.env`
    dotenvy::dotenv().ok();

    // Start the application
    cli::main::<App, Migrator>().await
}
