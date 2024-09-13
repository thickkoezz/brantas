use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::FriendGroup, friend_group};
use crate::dtos::friend_group::FriendGroupResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_friend_group(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  name: String,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = FriendGroup::delete_by_id((
        owner_id, name,
      )).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("group"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let friend_group = FriendGroup::find_by_id((
        owner_id, name,
      )).one(db).await?;
      if friend_group.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("group"))).into());
      }

      let mut friend_group: friend_group::ActiveModel = friend_group.unwrap().into();
      friend_group.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      friend_group.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_friend_group(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<FriendGroupResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let friend_groups = FriendGroup::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = friend_groups.into_iter()
        .map(|friend_group: friend_group::Model| FriendGroupResponse::from(friend_group))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let friend_groups = FriendGroup::find().all(db).await?;
      let res = friend_groups.into_iter()
        .map(|friend_group: friend_group::Model| FriendGroupResponse::from(friend_group))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
