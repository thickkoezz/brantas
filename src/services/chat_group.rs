use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::chat_group::{ChatGroupAddRequest, ChatGroupResponse};
use crate::entities::{chat_group, prelude::ChatGroup};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_chat_group(req: ChatGroupAddRequest) -> AppResult<ChatGroupResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = chat_group::ActiveModel {
    creator_id: Set(req.creator_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    name: Default::default(),
    is_public: Set(req.is_public),
    is_suspended: Set(false),
    is_channel: Set(req.is_channel),
  };
  let chat_group = ChatGroup::insert(model.clone()).exec(db).await?;
  Ok(ChatGroupResponse {
    creator_id: chat_group.last_insert_id.0,
    created_at: chat_group.last_insert_id.1,
    ..ChatGroupResponse::from(model)
  })
}

pub async fn delete_chat_group(
  deletion_mode: DeletionMode,
  creator_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = ChatGroup::delete_by_id((creator_id, created_at))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("chat_group"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let chat_group = ChatGroup::find_by_id((creator_id, created_at))
        .one(db)
        .await?;
      if chat_group.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("chat_group"))).into());
      }

      let mut chat_group: chat_group::ActiveModel = chat_group.unwrap().into();
      chat_group.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      chat_group.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_chat_group(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<ChatGroupResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let chat_groups = ChatGroup::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = chat_groups
        .into_iter()
        .map(|chat_group: chat_group::Model| ChatGroupResponse::from(chat_group))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let chat_groups = ChatGroup::find().all(db).await?;
      let res = chat_groups
        .into_iter()
        .map(|chat_group: chat_group::Model| ChatGroupResponse::from(chat_group))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
