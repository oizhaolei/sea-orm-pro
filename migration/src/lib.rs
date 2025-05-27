#![allow(
    elided_lifetimes_in_paths,
    clippy::wildcard_imports,
    clippy::enum_variant_names
)]
pub use sea_orm_migration::prelude::*;

mod m20241101_000001_address;
mod m20241101_000002_customer;
mod m20241101_000003_customer_address;
mod m20241101_000004_product_category;
mod m20241101_000005_product_description;
mod m20241101_000006_product_model;
mod m20241101_000007_product;
mod m20241101_000008_product_model_product_description;
mod m20241101_000009_sales_order_header;
mod m20241101_000010_sales_order_detail;
mod m20250101_000001_user;
mod m20250101_000002_seed_user;
mod m20250527_054828_create_bakery_table;
mod m20250527_070516_create_baker_table;
mod m20250527_070516_create_cake_baker_table;
mod m20250527_070516_create_cake_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241101_000001_address::Migration),
            Box::new(m20241101_000002_customer::Migration),
            Box::new(m20241101_000003_customer_address::Migration),
            Box::new(m20241101_000004_product_category::Migration),
            Box::new(m20241101_000005_product_description::Migration),
            Box::new(m20241101_000006_product_model::Migration),
            Box::new(m20241101_000007_product::Migration),
            Box::new(m20241101_000008_product_model_product_description::Migration),
            Box::new(m20241101_000009_sales_order_header::Migration),
            Box::new(m20241101_000010_sales_order_detail::Migration),
            Box::new(m20250101_000001_user::Migration),
            Box::new(m20250101_000002_seed_user::Migration),
            Box::new(m20250527_054828_create_bakery_table::Migration),
            Box::new(m20250527_070516_create_baker_table::Migration),
            Box::new(m20250527_070516_create_cake_table::Migration),
            Box::new(m20250527_070516_create_cake_baker_table::Migration),
        ]
    }
}
