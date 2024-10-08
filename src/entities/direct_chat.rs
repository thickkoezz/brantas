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
  pub replied_sender_id: Option<Uuid>,
  pub replied_receiver_id: Option<Uuid>,
  pub replied_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_sender_id: Option<Uuid>,
  pub forwarded_receiver_id: Option<Uuid>,
  pub forwarded_created_at: Option<DateTimeWithTimeZone>,
  pub forwarded_group_creator_id: Option<Uuid>,
  pub forwarded_group_created_at: Option<DateTimeWithTimeZone>,
  pub is_pinned: bool,
  pub pin_expired_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "Entity",
    from = "(Column::ForwardedSenderId, Column::ForwardedReceiverId, Column::ForwardedCreatedAt)",
    to = "(Column::SenderId, Column::ReceiverId, Column::CreatedAt)",
    on_update = "Cascade",
    on_delete = "SetNull"
  )]
  Forwarded,
  #[sea_orm(
    belongs_to = "Entity",
    from = "(Column::RepliedSenderId, Column::RepliedReceiverId, Column::RepliedCreatedAt)",
    to = "(Column::SenderId, Column::ReceiverId, Column::CreatedAt)",
    on_update = "Cascade",
    on_delete = "SetNull"
  )]
  Replied,
  #[sea_orm(has_many = "super::direct_chat_reaction::Entity")]
  DirectChatReaction,
  #[sea_orm(
    belongs_to = "super::group_chat::Entity",
    from = "(Column::ForwardedSenderId, Column::ForwardedGroupCreatorId, Column::ForwardedGroupCreatedAt, Column::ForwardedCreatedAt)",
    to = "(super::group_chat::Column::SenderId, super::group_chat::Column::GroupCreatorId, super::group_chat::Column::GroupCreatedAt, super::group_chat::Column::CreatedAt)",
    on_update = "Cascade",
    on_delete = "SetNull"
  )]
  GroupChat,
  #[sea_orm(has_many = "super::starred_chat::Entity")]
  StarredChat,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::ForwardedGroupCreatorId",
    to = "super::user_account::Column::Id",
    on_update = "Restrict",
    on_delete = "Cascade"
  )]
  ForwardedGroupCreator,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::ForwardedReceiverId",
    to = "super::user_account::Column::Id",
    on_update = "Restrict",
    on_delete = "Cascade"
  )]
  ForwardedReceiver,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::ForwardedSenderId",
    to = "super::user_account::Column::Id",
    on_update = "Restrict",
    on_delete = "Cascade"
  )]
  ForwardedSender,
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
    from = "Column::RepliedReceiverId",
    to = "super::user_account::Column::Id",
    on_update = "Restrict",
    on_delete = "Cascade"
  )]
  RepliedReceiver,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::RepliedSenderId",
    to = "super::user_account::Column::Id",
    on_update = "Restrict",
    on_delete = "Cascade"
  )]
  RepliedSender,
  #[sea_orm(
    belongs_to = "super::user_account::Entity",
    from = "Column::SenderId",
    to = "super::user_account::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  Sender,
}

impl Related<super::direct_chat_reaction::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::DirectChatReaction.def()
  }
}

impl Related<super::group_chat::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::GroupChat.def()
  }
}

impl Related<super::starred_chat::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::StarredChat.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "Entity", def = "Relation::Forwarded.def()")]
  Forwarded,
  #[sea_orm(entity = "Entity", def = "Relation::Replied.def()")]
  Replied,
  #[sea_orm(entity = "super::direct_chat_reaction::Entity")]
  DirectChatReaction,
  #[sea_orm(entity = "super::group_chat::Entity")]
  GroupChat,
  #[sea_orm(entity = "super::starred_chat::Entity")]
  StarredChat,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::ForwardedGroupCreator.def()"
  )]
  ForwardedGroupCreator,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::ForwardedReceiver.def()"
  )]
  ForwardedReceiver,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::ForwardedSender.def()"
  )]
  ForwardedSender,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::Receiver.def()"
  )]
  Receiver,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::RepliedReceiver.def()"
  )]
  RepliedReceiver,
  #[sea_orm(
    entity = "super::user_account::Entity",
    def = "Relation::RepliedSender.def()"
  )]
  RepliedSender,
  #[sea_orm(entity = "super::user_account::Entity", def = "Relation::Sender.def()")]
  Sender,
  #[sea_orm(entity = "Entity", def = "Relation::Forwarded.def().rev()")]
  Forward,
  #[sea_orm(entity = "Entity", def = "Relation::Replied.def().rev()")]
  Reply,
}
