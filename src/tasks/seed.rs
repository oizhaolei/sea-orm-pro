//! This task implements data seeding functionality for initializing new
//! development/demo environments.
//!
//! # Example
//!
//! Run the task with the following command:
//! ```sh
//! cargo run task
//! ```
//!
//! To override existing data and reset the data structure, use the following
//! command with the `refresh:true` argument:
//! ```sh
//! cargo run task seed_data refresh:true
//! ```

use encoding_rs::{UTF_16LE, WINDOWS_1252};
use loco_rs::{db, prelude::*};
use migration::Migrator;
use sea_orm::DbConn;
use serde::de::DeserializeOwned;

use crate::models::{self, baker, bakery, cake, cake_baker};

#[allow(clippy::module_name_repetitions)]
pub struct SeedData;
#[async_trait]
impl Task for SeedData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_data".to_string(),
            detail: "Task for seeding data".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let refresh = vars
            .cli_arg("refresh")
            .is_ok_and(|refresh| refresh == "true");
        let db = &app_context.db;

        // Run migration before seeding database
        if refresh {
            db::reset::<Migrator>(db).await?;
        } else {
            db::migrate::<Migrator>(db).await?;
        }

        // Some boilerplate
        const CRLF: csv::Terminator = csv::Terminator::CRLF;
        const DOLLAR_SIGN: csv::Terminator = csv::Terminator::Any(b'$');
        macro_rules! seed_table_win_1252 {
            ($entity: ident, $csv: expr) => {
                seed_table::<models::$entity::Model, _>($csv, WINDOWS_1252, b'\t', CRLF, db)
                    .await?;
            };
        }
        macro_rules! seed_table_utf_16 {
            ($entity: ident, $csv: expr) => {
                seed_table::<models::$entity::Model, _>($csv, UTF_16LE, b'\t', CRLF, db).await?;
            };
        }
        macro_rules! seed_table_utf_16_tilde {
            ($entity: ident, $csv: expr) => {
                seed_table::<models::$entity::Model, _>($csv, UTF_16LE, b'~', DOLLAR_SIGN, db)
                    .await?;
            };
        }

        // Seed each table in sequence
        seed_table_win_1252!(customer, "Customer.tsv");
        seed_table_win_1252!(address, "Address.tsv");
        seed_table_win_1252!(customer_address, "CustomerAddress.tsv");
        seed_table_win_1252!(sales_order_header, "SalesOrderHeader.tsv");
        seed_table_win_1252!(product_category, "ProductCategory.tsv");
        seed_table_utf_16_tilde!(product_model, "ProductModel.tsv");
        seed_table_win_1252!(product, "Product.tsv");
        seed_table_win_1252!(sales_order_detail, "SalesOrderDetail.tsv");
        seed_table_utf_16!(product_description, "ProductDescription.tsv");
        seed_table_win_1252!(
            product_model_product_description,
            "ProductModelProductDescription.tsv"
        );

        seed_bakery(db).await?;

        println!("All Tables Completed Seeding!");

        Ok(())
    }
}

async fn seed_table<T, A>(
    csv: &str,
    encoding: &'static encoding_rs::Encoding,
    delimiter: u8,
    terminator: csv::Terminator,
    db: &DbConn,
) -> Result<()>
where
    T: DeserializeOwned + ModelTrait + IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior,
    <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
{
    println!("Reading: {csv:?}");
    // Read data from CSV
    let file = std::fs::OpenOptions::new().read(true).open(format!(
        "{}/migration/AdventureWorks-2012-LT-Script/{}",
        env!("CARGO_MANIFEST_DIR"),
        csv
    ))?;
    let transcoded = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding))
        .build(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .terminator(terminator)
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(transcoded);
    // Collect into SeaORM's active model
    let mut active_models: Vec<A> = Vec::new();
    for result in rdr.deserialize() {
        let model: T = result.unwrap();
        active_models.push(model.into_active_model());
    }
    // Insert active model in chunks
    for models in active_models.chunks(100) {
        let models = models.iter().cloned();
        <A::Entity as EntityTrait>::insert_many(models)
            .exec(db)
            .await?;
    }
    println!("Seeding Completed: {csv:?}");
    Ok(())
}

async fn seed_bakery(db: &DbConn) -> Result<()> {
    let bakery = bakery::ActiveModel {
        name: Set("SeaSide Bakery".to_owned()),
        profit_margin: Set(10.4),
        ..Default::default()
    };
    let sea = bakery::Entity::insert(bakery)
        .exec(db)
        .await?
        .last_insert_id;

    let bakery = bakery::ActiveModel {
        name: Set("LakeSide Bakery".to_owned()),
        profit_margin: Set(5.8),
        ..Default::default()
    };
    let lake = bakery::Entity::insert(bakery)
        .exec(db)
        .await?
        .last_insert_id;

    let alice = baker::ActiveModel {
        name: Set("Alice".to_owned()),
        contact: Set("+44 15273388".to_owned()),
        bakery_id: Set(Some(sea)),
        ..Default::default()
    };
    let alice = baker::Entity::insert(alice).exec(db).await?.last_insert_id;

    let bob = baker::ActiveModel {
        name: Set("Bob".to_owned()),
        contact: Set("+852 12345678".to_owned()),
        bakery_id: Set(Some(lake)),
        ..Default::default()
    };
    let bob = baker::Entity::insert(bob).exec(db).await?.last_insert_id;

    let cake = cake::ActiveModel {
        name: Set("Chocolate Cake".to_owned()),
        price: Set("10.25".parse().unwrap()),
        gluten_free: Set(false),
        bakery_id: Set(sea),
        ..Default::default()
    };
    let choco = cake::Entity::insert(cake).exec(db).await?.last_insert_id;

    let mut cake = cake::ActiveModel {
        name: Set("Double Chocolate".to_owned()),
        price: Set("12.5".parse().unwrap()),
        gluten_free: Set(false),
        bakery_id: Set(sea),
        ..Default::default()
    };
    let double_1 = cake::Entity::insert(cake.clone())
        .exec(db)
        .await?
        .last_insert_id;
    cake.bakery_id = Set(lake);
    let double_2 = cake::Entity::insert(cake).exec(db).await?.last_insert_id;

    let mut cake = cake::ActiveModel {
        name: Set("Lemon Cake".to_owned()),
        price: Set("8.8".parse().unwrap()),
        gluten_free: Set(false),
        bakery_id: Set(sea),
        ..Default::default()
    };
    let lemon_1 = cake::Entity::insert(cake.clone())
        .exec(db)
        .await?
        .last_insert_id;
    cake.bakery_id = Set(lake);
    let lemon_2 = cake::Entity::insert(cake).exec(db).await?.last_insert_id;

    let mut cake = cake::ActiveModel {
        name: Set("Strawberry Cake".to_owned()),
        price: Set("9.9".parse().unwrap()),
        gluten_free: Set(false),
        bakery_id: Set(sea),
        ..Default::default()
    };
    let straw_1 = cake::Entity::insert(cake.clone())
        .exec(db)
        .await?
        .last_insert_id;
    cake.bakery_id = Set(lake);
    let straw_2 = cake::Entity::insert(cake).exec(db).await?.last_insert_id;

    let cake = cake::ActiveModel {
        name: Set("Orange Cake".to_owned()),
        price: Set("6.5".parse().unwrap()),
        gluten_free: Set(true),
        bakery_id: Set(lake),
        ..Default::default()
    };
    let orange = cake::Entity::insert(cake).exec(db).await?.last_insert_id;

    let mut cake = cake::ActiveModel {
        name: Set("New York Cheese".to_owned()),
        price: Set("12.5".parse().unwrap()),
        gluten_free: Set(false),
        bakery_id: Set(sea),
        ..Default::default()
    };
    let cheese_1 = cake::Entity::insert(cake.clone())
        .exec(db)
        .await?
        .last_insert_id;
    cake.bakery_id = Set(lake);
    let cheese_2 = cake::Entity::insert(cake).exec(db).await?.last_insert_id;

    let mut cake = cake::ActiveModel {
        name: Set("Blueburry Cheese".to_owned()),
        price: Set("11.5".parse().unwrap()),
        gluten_free: Set(true),
        bakery_id: Set(sea),
        ..Default::default()
    };
    let blue_1 = cake::Entity::insert(cake.clone())
        .exec(db)
        .await?
        .last_insert_id;
    cake.bakery_id = Set(lake);
    let blue_2 = cake::Entity::insert(cake).exec(db).await?.last_insert_id;

    let rel = cake_baker::ActiveModel {
        cake_id: Set(choco),
        baker_id: Set(alice),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;

    let rel = cake_baker::ActiveModel {
        cake_id: Set(double_1),
        baker_id: Set(alice),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;
    let rel = cake_baker::ActiveModel {
        cake_id: Set(double_2),
        baker_id: Set(bob),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;

    let rel = cake_baker::ActiveModel {
        cake_id: Set(lemon_1),
        baker_id: Set(alice),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;
    let rel = cake_baker::ActiveModel {
        cake_id: Set(lemon_2),
        baker_id: Set(bob),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;

    let rel = cake_baker::ActiveModel {
        cake_id: Set(straw_1),
        baker_id: Set(alice),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;
    let rel = cake_baker::ActiveModel {
        cake_id: Set(straw_2),
        baker_id: Set(bob),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;

    let rel = cake_baker::ActiveModel {
        cake_id: Set(orange),
        baker_id: Set(bob),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;

    let rel = cake_baker::ActiveModel {
        cake_id: Set(cheese_1),
        baker_id: Set(alice),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;
    let rel = cake_baker::ActiveModel {
        cake_id: Set(cheese_2),
        baker_id: Set(bob),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;

    let rel = cake_baker::ActiveModel {
        cake_id: Set(blue_1),
        baker_id: Set(alice),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;
    let rel = cake_baker::ActiveModel {
        cake_id: Set(blue_2),
        baker_id: Set(bob),
    };
    cake_baker::Entity::insert(rel).exec(db).await?;

    Ok(())
}
