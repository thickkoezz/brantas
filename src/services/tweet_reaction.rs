use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::tweet_reaction::{TweetReactionAddRequest, TweetReactionResponse};
use crate::entities::{prelude::TweetReaction, tweet_reaction};
use crate::services::PaginatorOption;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_tweet_reaction(req: TweetReactionAddRequest) -> AppResult<TweetReactionResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = tweet_reaction::ActiveModel {
    owner_id: Set(req.owner_id),
    reacted_tweet_owner_id: Set(req.reacted_tweet_owner_id),
    reacted_tweet_created_at: Set(req.reacted_tweet_created_at),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    reaction_emoji: Set(req.reaction_emoji),
  };
  let tweet_reaction = TweetReaction::insert(model.clone()).exec(db).await?;
  Ok(TweetReactionResponse {
    owner_id: tweet_reaction.last_insert_id.0,
    reacted_tweet_owner_id: tweet_reaction.last_insert_id.1,
    reacted_tweet_created_at: tweet_reaction.last_insert_id.2,
    ..TweetReactionResponse::from(model)
  })
}

pub async fn delete_tweet_reaction(
  owner_id: Uuid,
  reacted_tweet_owner_id: Uuid,
  reacted_tweet_created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result =
    TweetReaction::delete_by_id((owner_id, reacted_tweet_owner_id, reacted_tweet_created_at))
      .exec(db)
      .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("reaction"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_tweet_reaction(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<TweetReactionResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let tweet_reactions = TweetReaction::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = tweet_reactions
        .into_iter()
        .map(|tweet_reaction: tweet_reaction::Model| TweetReactionResponse::from(tweet_reaction))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let tweet_reactions = TweetReaction::find().all(db).await?;
      let res = tweet_reactions
        .into_iter()
        .map(|tweet_reaction: tweet_reaction::Model| TweetReactionResponse::from(tweet_reaction))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
