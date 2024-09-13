use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Friend, friend};
use crate::dtos::friend::FriendResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_chat_group(
  deletion_mode: DeletionMode,
  invitor_id: Uuid,
  invitee_id: Uuid,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Friend::delete_by_id((invitor_id, invitee_id)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("friend"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let friend = Friend::find_by_id((
        invitor_id, invitee_id,
      )).one(db).await?;
      if friend.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("friend"))).into());
      }

      let mut friend: friend::ActiveModel = friend.unwrap().into();
      friend.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      friend.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_friend(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<FriendResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let friends = Friend::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = friends.into_iter()
        .map(|friend: friend::Model| FriendResponse::from(friend)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let friends = Friend::find().all(db).await?;
      let res = friends.into_iter()
        .map(|friend: friend::Model| FriendResponse::from(friend)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
