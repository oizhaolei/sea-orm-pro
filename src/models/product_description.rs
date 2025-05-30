//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "product_description")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub product_description_id: i32,
    pub description: String,
    #[sea_orm(unique)]
    pub rowguid: Uuid,
    #[serde(deserialize_with = "super::utils::date_time_from_str")]
    pub created_date: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::product_model_product_description::Entity")]
    ProductModelProductDescription,
}

impl Related<super::product_model_product_description::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductModelProductDescription.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::product_model_product_description::Entity")]
    ProductModelProductDescription,
}
