//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "chat_group_member")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub group_creator_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub group_created_at: DateTimeWithTimeZone,
  #[sea_orm(primary_key, auto_increment = false)]
  pub member_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::chat_group::Entity",
    from = "(Column::GroupCreatorId, Column::GroupCreatedAt)",
    to = "(super::chat_group::Column::CreatorId, super::chat_group::Column::CreatedAt)",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  ChatGroup,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::MemberId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Member,
}

impl Related<super::chat_group::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ChatGroup.def()
  }
}

impl Related<super::user_account::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Member.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::chat_group::Entity")]
  ChatGroup,
  #[sea_orm(entity = "super::user_account::Entity")]
  Member,
}
