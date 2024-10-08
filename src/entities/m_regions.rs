//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "m_regions")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: i16,
  pub name: String,
  #[sea_orm(column_type = "JsonBinary", nullable)]
  pub translations: Option<Json>,
  pub created_at: Option<DateTime>,
  pub updated_at: Option<DateTime>,
  pub flag: i16,
  pub wikidataid: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::m_countries::Entity")]
  MCountries,
  #[sea_orm(has_many = "super::m_subregions::Entity")]
  MSubregions,
}

impl Related<super::m_countries::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::MCountries.def()
  }
}

impl Related<super::m_subregions::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::MSubregions.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::m_countries::Entity")]
  MCountries,
  #[sea_orm(entity = "super::m_subregions::Entity")]
  MSubregions,
}
