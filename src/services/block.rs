use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Block, block};
use crate::dtos::block::BlockResponse;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_block(
  deletion_mode: DeletionMode,
  blocker_id: Uuid,
  target_id: Uuid,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Block::delete_by_id((blocker_id, target_id)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("block"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let block = Block::find_by_id((blocker_id, target_id)).one(db).await?;
      if block.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("block"))).into());
      }

      let mut block: block::ActiveModel = block.unwrap().into();
      block.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      block.update(db).await?;
      Ok(())
    }
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
      let res = blocks.into_iter().map(|block: block::Model| BlockResponse::from(block))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let blocks = Block::find().all(db).await?;
      let res = blocks.into_iter().map(|block: block::Model| BlockResponse::from(block))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
