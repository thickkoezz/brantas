//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "verification_token")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
  pub identifier: String,
  #[sea_orm(primary_key, auto_increment = false, column_type = "Text", unique)]
  pub token: String,
  pub expires: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}
