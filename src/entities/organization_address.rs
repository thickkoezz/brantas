//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "organization_address")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub organization_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub city_id: i32,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[sea_orm(column_type = "Text", nullable)]
  pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::m_cities::Entity",
    from = "Column::CityId",
    to = "super::m_cities::Column::Id",
    on_update = "Restrict",
    on_delete = "Restrict"
  )]
  MCities,
  #[sea_orm(
    belongs_to = "super::organization::Entity",
    from = "Column::OrganizationId",
    to = "super::organization::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Organization,
}

impl Related<super::m_cities::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::MCities.def()
  }
}

impl Related<super::organization::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Organization.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::m_cities::Entity")]
  MCities,
  #[sea_orm(entity = "super::organization::Entity")]
  Organization,
}
