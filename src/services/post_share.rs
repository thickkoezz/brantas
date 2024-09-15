use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::post_share::{PostShareAddRequest, PostShareResponse};
use crate::entities::{post_share, prelude::PostShare};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_post_share(req: PostShareAddRequest) -> AppResult<PostShareResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = post_share::ActiveModel {
    post_owner_id: Set(req.post_owner_id),
    post_created_at: Set(req.post_created_at),
    target_id: Set(req.target_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    can_comment: Set(req.can_comment),
    deleted_at: Set(None),
  };
  let post_share = PostShare::insert(model.clone()).exec(db).await?;
  Ok(PostShareResponse {
    post_owner_id: post_share.last_insert_id.0,
    post_created_at: post_share.last_insert_id.1,
    target_id: post_share.last_insert_id.2,
    ..PostShareResponse::from(model)
  })
}

pub async fn delete_post_share(
  deletion_mode: DeletionMode,
  post_owner_id: Uuid,
  post_created_at: DateTimeWithTimeZone,
  target_id: Uuid,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = PostShare::delete_by_id((post_owner_id, post_created_at, target_id))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("user"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let post_share = PostShare::find_by_id((post_owner_id, post_created_at, target_id))
        .one(db)
        .await?;
      if post_share.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
      }

      let mut post_share: post_share::ActiveModel = post_share.unwrap().into();
      post_share.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      post_share.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_post_share(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<PostShareResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let post_shares = PostShare::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = post_shares
        .into_iter()
        .map(|post_share: post_share::Model| PostShareResponse::from(post_share))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let post_shares = PostShare::find().all(db).await?;
      let res = post_shares
        .into_iter()
        .map(|post_share: post_share::Model| PostShareResponse::from(post_share))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
