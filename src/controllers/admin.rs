use loco_rs::{environment::Environment, prelude::*};
use migration::{IntoColumnRef, IntoIden};
use sea_orm::{
    prelude::DateTime,
    sea_query::{Alias, Asterisk, Expr, Func, SimpleExpr},
    DatabaseBackend, DbConn, DeriveCustomColumn, FromQueryResult, IdenStatic,
    QueryOrder, QuerySelect,
};
use sea_orm_pro::{ConfigParser, JsonCfg};
use seaography::lazy_static;
use serde::{Deserialize, Serialize};

use crate::models::{customer, product, product_category, sales_order_detail, sales_order_header};

const CONFIG_ROOT: &str = "pro_admin";

lazy_static::lazy_static! {
    static ref CONFIG: JsonCfg = ConfigParser::new().load_config(CONFIG_ROOT).unwrap();
}

pub async fn config(State(ctx): State<AppContext>) -> Result<Response> {
    if ctx.environment == Environment::Production {
        // Release: load config from the disk once and then return the cached config afterwards
        format::json(&*CONFIG)
    } else {
        // Debug: load config from disk on every request
        let config = ConfigParser::new()
            .load_config(CONFIG_ROOT)
            .map_err(Into::<Box<dyn std::error::Error + Send + Sync>>::into)?;
        format::json(config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DashboardBody {
    pub graph: String,
    pub from: Option<DateTime>,
    pub to: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, PartialEq)]
pub struct Datum {
    pub key: String,
    pub val: i32,
}

#[derive(Copy, Clone, Debug, DeriveCustomColumn)]
pub enum DatumColumn {
    Key,
    Val,
}

impl IdenStatic for DatumColumn {
    fn as_str(&self) -> &str {
        self.default_as_str()
    }
}

pub async fn dashboard(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(body): Json<DashboardBody>,
) -> Result<Response> {
    let db = &ctx.db;
    let data = match body.graph.as_str() {
        "new_customer_by_month" => {
            customer::Entity::find()
                .select_only()
                .column_as(
                    Expr::expr(cast_as_year_month(db, customer::Column::CreatedDate)),
                    DatumColumn::Key,
                )
                .column_as(
                    Expr::expr(Func::cast_as(
                        Func::count(Expr::col(Asterisk)),
                        int_keyword(db),
                    )),
                    DatumColumn::Val,
                )
                .filter(customer::Column::CreatedDate.gte(body.from.unwrap()))
                .filter(customer::Column::CreatedDate.lte(body.to.unwrap()))
                .group_by(Expr::col(DatumColumn::Key))
                .into_model::<Datum>()
                .all(db)
                .await?
        }
        "sales_value_by_day" => {
            sales_order_detail::Entity::find()
                .select_only()
                .column_as(
                    Expr::expr(
                        cast_as_day(
                            db,
                            (
                                sales_order_header::Entity,
                                sales_order_header::Column::OrderDate,
                            ),
                        )
                    ),
                    DatumColumn::Key,
                )
                .column_as(
                    Expr::expr(Func::cast_as(
                        Func::sum(
                            Expr::col(sales_order_detail::Column::UnitPrice)
                                .mul(Expr::col(sales_order_detail::Column::OrderQty)),
                        ),
                        int_keyword(db),
                    )),
                    DatumColumn::Val,
                )
                .left_join(sales_order_header::Entity)
                .filter(
                    Expr::col((
                        sales_order_header::Entity,
                        sales_order_header::Column::OrderDate,
                    ))
                    .gte(body.from.unwrap()),
                )
                .filter(
                    Expr::col((
                        sales_order_header::Entity,
                        sales_order_header::Column::OrderDate,
                    ))
                    .lte(body.to.unwrap()),
                )
                .group_by(Expr::col(DatumColumn::Key))
                .into_model::<Datum>()
                .all(db)
                .await?
        }
        "product_by_product_category" => {
            product_category::Entity::find()
                .select_only()
                .column_as(
                    Expr::expr(Expr::col((
                        product_category::Entity,
                        product_category::Column::Name,
                    ))),
                    DatumColumn::Key,
                )
                .column_as(
                    Expr::expr(Func::cast_as(
                        Func::count(Expr::col(Asterisk)),
                        int_keyword(db),
                    )),
                    DatumColumn::Val,
                )
                .left_join(product::Entity)
                .group_by(Expr::col(DatumColumn::Key))
                .order_by_desc(Expr::col(DatumColumn::Val))
                .into_model::<Datum>()
                .all(db)
                .await?
        }
        _ => not_found()?,
    };
    format::json(data)
}

fn cast_as_year_month(db: &DbConn, col: impl IntoColumnRef) -> SimpleExpr {
    let func = match db.get_database_backend() {
        DatabaseBackend::MySql => Func::cust(Alias::new("DATE_FORMAT"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("%Y-%m"),
        DatabaseBackend::Postgres => Func::cust(Alias::new("TO_CHAR"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("YYYY-mm"),
        DatabaseBackend::Sqlite => Func::cust(Alias::new("STRFTIME"))
            .arg("%Y-%m")
            .arg(Expr::col(col.into_column_ref())),
    };
    func.into()
}

fn cast_as_day(db: &DbConn, col: impl IntoColumnRef) -> SimpleExpr {
    let func = match db.get_database_backend() {
        DatabaseBackend::MySql => Func::cust(Alias::new("DATE_FORMAT"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("%Y-%m-%d"),
        DatabaseBackend::Postgres => Func::cust(Alias::new("TO_CHAR"))
            .arg(Expr::col(col.into_column_ref()))
            .arg("YYYY-mm-dd"),
        DatabaseBackend::Sqlite => Func::cust(Alias::new("STRFTIME"))
            .arg("%Y-%m-%d")
            .arg(Expr::col(col.into_column_ref())),
    };
    func.into()
}

fn int_keyword(db: &DbConn) -> impl IntoIden {
    match db.get_database_backend() {
        DatabaseBackend::MySql => Alias::new("SIGNED INTEGER"),
        DatabaseBackend::Postgres => Alias::new("INT4"),
        DatabaseBackend::Sqlite => Alias::new("INT"),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        // Admin route prefix
        .prefix("admin")
        // Fetch web config
        .add("/config", get(config))
        // Fetch dashboard graph data
        .add("/dashboard", post(dashboard))
}
