use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::group_chat_reaction::{GroupChatReactionAddRequest, GroupChatReactionResponse};
use crate::entities::{group_chat_reaction, prelude::GroupChatReaction};
use crate::services::PaginatorOption;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait};
use uuid::Uuid;

pub async fn add_group_chat_reaction(
  req: GroupChatReactionAddRequest,
) -> AppResult<GroupChatReactionResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = group_chat_reaction::ActiveModel {
    owner_id: Set(req.owner_id),
    reacted_group_chat_sender_id: Set(req.reacted_group_chat_sender_id),
    reacted_group_chat_group_creator_id: Set(req.reacted_group_chat_sender_id),
    reacted_group_chat_group_created_at: Set(req.reacted_group_chat_group_created_at),
    reacted_group_chat_created_at: Set(req.reacted_group_chat_created_at),
    created_at: Default::default(),
    reaction_emoji: Default::default(),
  };
  let group_chat_reaction = GroupChatReaction::insert(model.clone()).exec(db).await?;
  Ok(GroupChatReactionResponse {
    owner_id: group_chat_reaction.last_insert_id.0,
    reacted_group_chat_sender_id: group_chat_reaction.last_insert_id.1,
    reacted_group_chat_group_creator_id: group_chat_reaction.last_insert_id.2,
    reacted_group_chat_group_created_at: group_chat_reaction.last_insert_id.3,
    created_at: group_chat_reaction.last_insert_id.4,
    ..GroupChatReactionResponse::from(model)
  })
}

pub async fn delete_group_chat_reaction(
  owner_id: Uuid,
  reacted_group_chat_sender_id: Uuid,
  reacted_group_chat_group_creator_id: Uuid,
  reacted_group_chat_group_created_at: DateTimeWithTimeZone,
  reacted_group_chat_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = GroupChatReaction::delete_by_id((
    owner_id,
    reacted_group_chat_sender_id,
    reacted_group_chat_group_creator_id,
    reacted_group_chat_group_created_at,
    reacted_group_chat_created_at,
  ))
  .exec(db)
  .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("reaction"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_group_chat_reaction(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<GroupChatReactionResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let group_chat_reactions = GroupChatReaction::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = group_chat_reactions
        .into_iter()
        .map(|group_chat_reaction: group_chat_reaction::Model| {
          GroupChatReactionResponse::from(group_chat_reaction)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let group_chat_reactions = GroupChatReaction::find().all(db).await?;
      let res = group_chat_reactions
        .into_iter()
        .map(|group_chat_reaction: group_chat_reaction::Model| {
          GroupChatReactionResponse::from(group_chat_reaction)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
