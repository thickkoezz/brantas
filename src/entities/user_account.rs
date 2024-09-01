//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_account")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub owner_id: Uuid,
  #[sea_orm(unique)]
  pub email: String,
  #[sea_orm(unique)]
  pub username: Option<String>,
  pub picture: Option<String>,
  pub password: Option<String>,
  pub salt: Option<String>,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: Option<DateTimeWithTimeZone>,
  pub deleted_at: Option<DateTimeWithTimeZone>,
  #[sea_orm(column_type = "Decimal(Some((18, 2)))")]
  pub balance: Decimal,
  pub is_super_admin: bool,
  #[sea_orm(column_type = "Text")]
  pub r#type: String,
  #[sea_orm(column_type = "Text")]
  pub provider: String,
  #[sea_orm(column_type = "Text")]
  pub provider_account_id: String,
  #[sea_orm(column_type = "Text", nullable)]
  pub refresh_token: Option<String>,
  #[sea_orm(column_type = "Text", nullable)]
  pub access_token: Option<String>,
  #[sea_orm(column_type = "Text", nullable)]
  pub token_type: Option<String>,
  #[sea_orm(column_type = "Text", nullable)]
  pub scope: Option<String>,
  #[sea_orm(column_type = "Text", nullable)]
  pub id_token: Option<String>,
  #[sea_orm(column_type = "Text", nullable)]
  pub session_state: Option<String>,
  pub expires_at: Option<DateTimeWithTimeZone>,
  pub refresh_token_expires_in: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::balance_history::Entity")]
  BalanceHistory,
  #[sea_orm(has_many = "super::chat_group::Entity")]
  ChatGroup,
  #[sea_orm(has_many = "super::chat_group_member::Entity")]
  ChatGroupMember,
  #[sea_orm(has_many = "super::email::Entity")]
  Email,
  #[sea_orm(has_many = "super::friend_group::Entity")]
  FriendGroup,
  #[sea_orm(has_many = "super::link::Entity")]
  Link,
  #[sea_orm(has_many = "super::note::Entity")]
  Note,
  #[sea_orm(
    belongs_to = "super::person::Entity",
    from = "Column::OwnerId",
    to = "super::person::Column::Id",
    on_update = "Restrict",
    on_delete = "Restrict"
  )]
  Person,
  #[sea_orm(has_many = "super::phone::Entity")]
  Phone,
  #[sea_orm(has_many = "super::photo::Entity")]
  Photo,
  #[sea_orm(has_many = "super::post::Entity")]
  Post,
  #[sea_orm(has_many = "super::session::Entity")]
  Session,
  #[sea_orm(has_many = "super::socmed_url::Entity")]
  SocmedUrl,
  #[sea_orm(has_many = "super::tweet::Entity")]
  Tweet,
  #[sea_orm(has_many = "super::video::Entity")]
  Video,
}

impl Related<super::balance_history::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::BalanceHistory.def()
  }
}

impl Related<super::chat_group::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ChatGroup.def()
  }
}

impl Related<super::chat_group_member::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::ChatGroupMember.def()
  }
}

impl Related<super::email::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Email.def()
  }
}

impl Related<super::friend_group::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::FriendGroup.def()
  }
}

impl Related<super::link::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Link.def()
  }
}

impl Related<super::note::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Note.def()
  }
}

impl Related<super::person::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Person.def()
  }
}

impl Related<super::phone::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Phone.def()
  }
}

impl Related<super::photo::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Photo.def()
  }
}

impl Related<super::post::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Post.def()
  }
}

impl Related<super::session::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Session.def()
  }
}

impl Related<super::socmed_url::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::SocmedUrl.def()
  }
}

impl Related<super::tweet::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Tweet.def()
  }
}

impl Related<super::video::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Video.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::balance_history::Entity")]
  BalanceHistory,
  #[sea_orm(entity = "super::chat_group::Entity")]
  ChatGroup,
  #[sea_orm(entity = "super::chat_group_member::Entity")]
  ChatGroupMember,
  #[sea_orm(entity = "super::email::Entity")]
  Email,
  #[sea_orm(entity = "super::friend_group::Entity")]
  FriendGroup,
  #[sea_orm(entity = "super::link::Entity")]
  Link,
  #[sea_orm(entity = "super::note::Entity")]
  Note,
  #[sea_orm(entity = "super::person::Entity")]
  Person,
  #[sea_orm(entity = "super::phone::Entity")]
  Phone,
  #[sea_orm(entity = "super::photo::Entity")]
  Photo,
  #[sea_orm(entity = "super::post::Entity")]
  Post,
  #[sea_orm(entity = "super::session::Entity")]
  Session,
  #[sea_orm(entity = "super::socmed_url::Entity")]
  SocmedUrl,
  #[sea_orm(entity = "super::tweet::Entity")]
  Tweet,
  #[sea_orm(entity = "super::video::Entity")]
  Video,
}
