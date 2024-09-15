//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "job_project")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub organization_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub person_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub job_created_at: DateTimeWithTimeZone,
  #[sea_orm(primary_key, auto_increment = false)]
  pub project_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[sea_orm(column_type = "Text", nullable)]
  pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::job::Entity",
    from = "(Column::OrganizationId, Column::PersonId, Column::JobCreatedAt)",
    to = "(super::job::Column::OrganizationId, super::job::Column::PersonId, super::job::Column::CreatedAt)",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  Job,
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
    from = "Column::PersonId",
    to = "super::person::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Person,
  #[sea_orm(
    belongs_to = "super::project::Entity",
    from = "(Column::PersonId, Column::ProjectCreatedAt)",
    to = "(super::project::Column::PersonId, super::project::Column::CreatedAt)",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  Project,
}

impl Related<super::job::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Job.def()
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

impl Related<super::project::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Project.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::job::Entity")]
  Job,
  #[sea_orm(entity = "super::organization::Entity")]
  Organization,
  #[sea_orm(entity = "super::person::Entity")]
  Person,
  #[sea_orm(entity = "super::project::Entity")]
  Project,
}
