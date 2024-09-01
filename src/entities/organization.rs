//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "organization")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub name: String,
  pub abbreviation: Option<String>,
  #[sea_orm(column_type = "Text", nullable)]
  pub description: Option<String>,
  pub dob: Option<Date>,
  pub dead_at: Option<Date>,
  #[sea_orm(column_type = "JsonBinary", nullable)]
  pub extra_info: Option<Json>,
  pub is_dead: bool,
  pub parent_id: Option<Uuid>,
  pub logo: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::department::Entity")]
  Department,
  #[sea_orm(has_many = "super::job::Entity")]
  Job,
  #[sea_orm(has_many = "super::job_project::Entity")]
  JobProject,
  #[sea_orm(has_many = "super::job_skill::Entity")]
  JobSkill,
  #[sea_orm(
    belongs_to = "Entity",
    from = "Column::ParentId",
    to = "Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  SelfRef,
  #[sea_orm(has_many = "super::organization_administrator::Entity")]
  OrganizationAdministrator,
}

impl Related<super::department::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Department.def()
  }
}

impl Related<super::job::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Job.def()
  }
}

impl Related<super::job_project::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::JobProject.def()
  }
}

impl Related<super::job_skill::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::JobSkill.def()
  }
}

impl Related<super::organization_administrator::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::OrganizationAdministrator.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::department::Entity")]
  Department,
  #[sea_orm(entity = "super::job::Entity")]
  Job,
  #[sea_orm(entity = "super::job_project::Entity")]
  JobProject,
  #[sea_orm(entity = "super::job_skill::Entity")]
  JobSkill,
  #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def()")]
  SelfRef,
  #[sea_orm(entity = "super::organization_administrator::Entity")]
  OrganizationAdministrator,
  #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def().rev()")]
  SelfRefReverse,
}
