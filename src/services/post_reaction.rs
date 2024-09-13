use crate::services::PaginatorOption;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::PostReaction, post_reaction};
use crate::dtos::post_reaction::PostReactionResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait};
use uuid::Uuid;

pub async fn delete_post_reaction(
  owner_id: Uuid,
  reacted_post_owner_id: Uuid,
  reacted_post_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = PostReaction::delete_by_id((
    owner_id,
    reacted_post_owner_id,
    reacted_post_created_at,
  ))
    .exec(db)
    .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("reaction"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_post_reaction(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<PostReactionResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let post_reactions = PostReaction::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = post_reactions.into_iter()
        .map(|post_reaction: post_reaction::Model| PostReactionResponse::from(post_reaction))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let post_reactions = PostReaction::find().all(db).await?;
      let res = post_reactions.into_iter()
        .map(|post_reaction: post_reaction::Model| PostReactionResponse::from(post_reaction))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
