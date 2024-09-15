use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::direct_chat_reaction::{DirectChatReactionAddRequest, DirectChatReactionResponse};
use crate::entities::{direct_chat_reaction, prelude::DirectChatReaction};
use crate::services::PaginatorOption;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_direct_chat_reaction(
  req: DirectChatReactionAddRequest,
) -> AppResult<DirectChatReactionResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = direct_chat_reaction::ActiveModel {
    owner_id: Set(req.owner_id),
    reacted_direct_chat_sender_id: Set(req.reacted_direct_chat_sender_id),
    reacted_direct_chat_receiver_id: Set(req.reacted_direct_chat_receiver_id),
    reacted_direct_chat_created_at: Set(req.reacted_direct_chat_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    reaction_emoji: Set(req.reaction_emoji),
  };
  let direct_chat_reaction = DirectChatReaction::insert(model.clone()).exec(db).await?;
  Ok(DirectChatReactionResponse {
    owner_id: direct_chat_reaction.last_insert_id.0,
    reacted_direct_chat_sender_id: direct_chat_reaction.last_insert_id.1,
    reacted_direct_chat_receiver_id: direct_chat_reaction.last_insert_id.2,
    reacted_direct_chat_created_at: direct_chat_reaction.last_insert_id.3,
    ..DirectChatReactionResponse::from(model)
  })
}

pub async fn delete_direct_chat_reaction(
  owner_id: Uuid,
  reacted_direct_chat_sender_id: Uuid,
  reacted_direct_chat_receiver_id: Uuid,
  reacted_direct_chat_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = DirectChatReaction::delete_by_id((
    owner_id,
    reacted_direct_chat_sender_id,
    reacted_direct_chat_receiver_id,
    reacted_direct_chat_created_at,
  ))
  .exec(db)
  .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("reaction"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_direct_chat_reaction(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<DirectChatReactionResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let direct_chat_reactions = DirectChatReaction::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = direct_chat_reactions
        .into_iter()
        .map(|direct_chat_reaction: direct_chat_reaction::Model| {
          DirectChatReactionResponse::from(direct_chat_reaction)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let direct_chat_reactions = DirectChatReaction::find().all(db).await?;
      let res = direct_chat_reactions
        .into_iter()
        .map(|direct_chat_reaction: direct_chat_reaction::Model| {
          DirectChatReactionResponse::from(direct_chat_reaction)
        })
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
