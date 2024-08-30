//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "organization_administrator")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub organization_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub administrator_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub department_id: Uuid,
  pub created_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::department::Entity",
    from = "Column::DepartmentId",
    to = "super::department::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Department,
  #[sea_orm(
    belongs_to = "super::organization::Entity",
    from = "Column::OrganizationId",
    to = "super::organization::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Organization,
  #[sea_orm(
    belongs_to = "super::person::Entity",
    from = "Column::AdministratorId",
    to = "super::person::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Person,
}

impl Related<super::department::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Department.def()
  }
}

impl Related<super::organization::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Organization.def()
  }
}

impl Related<super::person::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Person.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::department::Entity")]
  Department,
  #[sea_orm(entity = "super::organization::Entity")]
  Organization,
  #[sea_orm(entity = "super::person::Entity")]
  Person,
}
