//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "starred_chat")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub creator_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub created_at: DateTimeWithTimeZone,
  pub chat_sender_id: Option<Uuid>,
  pub direct_chat_receiver_id: Option<Uuid>,
  pub group_chat_group_creator_id: Option<Uuid>,
  pub group_chat_group_created_at: Option<DateTimeWithTimeZone>,
  pub chat_created_at: Option<DateTimeWithTimeZone>,
  pub expire_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::direct_chat::Entity",
    from = "(Column::ChatSenderId, Column::DirectChatReceiverId, Column::ChatCreatedAt)",
    to = "(super::direct_chat::Column::SenderId, super::direct_chat::Column::ReceiverId, super::direct_chat::Column::CreatedAt)",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  DirectChat,
  #[sea_orm(
    belongs_to = "super::group_chat::Entity",
    from = "(Column::ChatSenderId, Column::GroupChatGroupCreatorId, Column::GroupChatGroupCreatedAt, Column::ChatCreatedAt)",
    to = "(super::group_chat::Column::SenderId, super::group_chat::Column::GroupCreatorId, super::group_chat::Column::GroupCreatedAt, super::group_chat::Column::CreatedAt)",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  GroupChat,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::ChatSenderId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  ChatSender,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::CreatorId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Creator,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::DirectChatReceiverId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  DirectChatReceiver,
}

impl Related<super::direct_chat::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::DirectChat.def()
  }
}

impl Related<super::group_chat::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::GroupChat.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::direct_chat::Entity")]
  DirectChat,
  #[sea_orm(entity = "super::group_chat::Entity")]
  GroupChat,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::ChatSender.def()"
  )]
  ChatSender,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::Creator.def()"
  )]
  Creator,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::DirectChatReceiver.def()"
  )]
  DirectChatReceiver,
}
