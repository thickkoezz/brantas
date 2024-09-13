use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::{Block, Email};
use crate::entities::{block, email};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use crate::dtos::block::BlockResponse;

pub async fn delete_email(deletion_mode: DeletionMode, email: String) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Email::delete_by_id(email).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("email"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let email = Email::find_by_id(email).one(db).await?;
      if email.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("email"))).into());
      }

      let mut email: email::ActiveModel = email.unwrap().into();
      email.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      email.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_block(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<BlockResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let blocks = Block::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = blocks.into_iter()
        .map(|block: block::Model| BlockResponse::from(block)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let blocks = Block::find().all(db).await?;
      let res = blocks.into_iter()
        .map(|block: block::Model| BlockResponse::from(block)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
