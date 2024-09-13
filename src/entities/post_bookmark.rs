//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::post::Entity",
    from = "(Column::BookmarkedPostOwnerId, Column::BookmarkedPostCreatedAt)",
    to = "(super::post::Column::OwnerId, super::post::Column::CreatedAt)",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Post,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::OwnerId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Owner,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::BookmarkedPostOwnerId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  BookmarkedPostOwner,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post_bookmark")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub owner_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub bookmarked_post_owner_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub bookmarked_post_created_at: DateTimeWithTimeZone,
  pub created_at: DateTimeWithTimeZone,
}

impl Related<super::post::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Post.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::post::Entity")]
  Post,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::Owner.def()"
  )]
  Owner,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::BookmarkedPostOwner.def()"
  )]
  BookmarkedPostOwner,
}
