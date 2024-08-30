//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "block")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub blocker_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub target_id: Uuid,
  pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::BlockerId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  UserAccount2,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::TargetId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  UserAccount1,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::UserAccount2.def()"
  )]
  UserAccount2,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::UserAccount1.def()"
  )]
  UserAccount1,
}
