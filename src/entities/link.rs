//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "link")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  pub link_url: String,
  pub hashtag: Option<String>,
  pub use_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::link_code::Entity")]
  LinkCode,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::OwnerId",
    to = "super::user_account::Column::Id",
    on_update = "Restrict",
    on_delete = "Cascade"
  )]
  UserAccount,
}

impl Related<super::link_code::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::LinkCode.def()
  }
}

impl Related<super::user_account::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::UserAccount.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::link_code::Entity")]
  LinkCode,
  #[sea_orm(entity = "super::user_account::Entity")]
  UserAccount,
}
