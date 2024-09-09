//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "direct_chat")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub sender_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub receiver_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub created_at: DateTimeWithTimeZone,
  pub content: String,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::ReceiverId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Receiver,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::SenderId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Sender,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::Receiver.def()"
  )]
  Receiver,
  #[sea_orm(entity = "super::user_account::Entity", def = "Relation::Sender.def()")]
  Sender,
}
