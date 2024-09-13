use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::PostShare, post_share};
use crate::dtos::post_share::PostShareResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_post_share(
  deletion_mode: DeletionMode,
  post_owner_id: Uuid,
  post_created_at: DateTimeWithTimeZone,
  target_id: Uuid,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = PostShare::delete_by_id((
        post_owner_id, post_created_at, target_id,
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("user"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let post_share = PostShare::find_by_id((
        post_owner_id, post_created_at, target_id,
      )).one(db).await?;
      if post_share.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("user"))).into());
      }

      let mut post_share: post_share::ActiveModel = post_share.unwrap().into();
      post_share.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      post_share.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_post_share(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<PostShareResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let post_shares = PostShare::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = post_shares.into_iter()
        .map(|post_share: post_share::Model| PostShareResponse::from(post_share))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let post_shares = PostShare::find().all(db).await?;
      let res = post_shares.into_iter()
        .map(|post_share: post_share::Model| PostShareResponse::from(post_share))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
