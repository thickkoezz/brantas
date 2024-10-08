//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub owner_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub title: String,
  #[sea_orm(column_type = "Text")]
  pub content: String,
  pub is_published: bool,
  pub hashtag: Option<String>,
  pub view_count: i32,
  pub comment_count: i32,
  pub reaction_count: i32,
  pub is_public: bool,
  pub group_name: Option<String>,
  pub can_comment: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::friend_group::Entity",
    from = "(Column::OwnerId, Column::GroupName)",
    to = "(super::friend_group::Column::OwnerId, super::friend_group::Column::Name)",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  FriendGroup,
  #[sea_orm(has_many = "super::post_bookmark::Entity")]
  PostBookmark,
  #[sea_orm(has_many = "super::post_comment::Entity")]
  PostComment,
  #[sea_orm(has_many = "super::post_reaction::Entity")]
  PostReaction,
  #[sea_orm(has_many = "super::post_share::Entity")]
  PostShare,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::OwnerId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Owner,
}

impl Related<super::friend_group::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::FriendGroup.def()
  }
}

impl Related<super::post_bookmark::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PostBookmark.def()
  }
}

impl Related<super::post_comment::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PostComment.def()
  }
}

impl Related<super::post_reaction::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PostReaction.def()
  }
}

impl Related<super::post_share::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PostShare.def()
  }
}

impl Related<super::user_account::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Owner.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::friend_group::Entity")]
  FriendGroup,
  #[sea_orm(entity = "super::post_bookmark::Entity")]
  PostBookmark,
  #[sea_orm(entity = "super::post_comment::Entity")]
  PostComment,
  #[sea_orm(entity = "super::post_reaction::Entity")]
  PostReaction,
  #[sea_orm(entity = "super::post_share::Entity")]
  PostShare,
  #[sea_orm(entity = "super::user_account::Entity")]
  Owner,
}
