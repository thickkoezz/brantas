use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::post_reaction::PostReactionDTO;
use crate::entities::{post_reaction, prelude::PostReaction};
use crate::services::PaginatorOption;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_post_reaction(req: PostReactionDTO) -> AppResult<PostReactionDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = post_reaction::ActiveModel {
    owner_id: Set(req.owner_id),
    reacted_post_owner_id: Set(req.reacted_post_owner_id),
    reacted_post_created_at: Set(req.reacted_post_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    reaction_emoji: Set(req.reaction_emoji),
  };
  let post_reaction = PostReaction::insert(model.clone()).exec(db).await?;
  Ok(PostReactionDTO {
    owner_id: post_reaction.last_insert_id.0,
    reacted_post_owner_id: post_reaction.last_insert_id.1,
    reacted_post_created_at: post_reaction.last_insert_id.2,
    ..PostReactionDTO::from(model)
  })
}

pub async fn delete_post_reaction(
  owner_id: Uuid,
  reacted_post_owner_id: Uuid,
  reacted_post_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result =
    PostReaction::delete_by_id((owner_id, reacted_post_owner_id, reacted_post_created_at))
      .exec(db)
      .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("reaction"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_post_reaction(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<PostReactionDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let post_reactions = PostReaction::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = post_reactions
        .into_iter()
        .map(|post_reaction: post_reaction::Model| PostReactionDTO::from(post_reaction))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let post_reactions = PostReaction::find().all(db).await?;
      let res = post_reactions
        .into_iter()
        .map(|post_reaction: post_reaction::Model| PostReactionDTO::from(post_reaction))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
