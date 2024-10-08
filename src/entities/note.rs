//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "note")]
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::note_edit_history::Entity")]
  NoteEditHistory,
  #[sea_orm(has_many = "super::note_share::Entity")]
  NoteShare,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::OwnerId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Owner,
}

impl Related<super::note_edit_history::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::NoteEditHistory.def()
  }
}

impl Related<super::note_share::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::NoteShare.def()
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
  #[sea_orm(entity = "super::note_edit_history::Entity")]
  NoteEditHistory,
  #[sea_orm(entity = "super::note_share::Entity")]
  NoteShare,
  #[sea_orm(entity = "super::user_account::Entity")]
  Owner,
}
