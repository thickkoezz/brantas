//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "photo_comment")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub owner_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub created_at: DateTimeWithTimeZone,
  pub commented_photo_owner_id: Uuid,
  pub commented_photo_created_at: DateTimeWithTimeZone,
  pub commented_photo: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub content: String,
  pub reaction_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::photo::Entity",
    from = "(Column::CommentedPhotoOwnerId, Column::CommentedPhotoCreatedAt, Column::CommentedPhoto)",
    to = "(super::photo::Column::OwnerId, super::photo::Column::CreatedAt, super::photo::Column::Photo)",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Photo,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::CommentedPhotoOwnerId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  CommentedPhotoOwner,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::OwnerId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Owner,
}

impl Related<super::photo::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Photo.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::photo::Entity")]
  Photo,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::CommentedPhotoOwner.def()"
  )]
  CommentedPhotoOwner,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::Owner.def()"
  )]
  Owner,
}
