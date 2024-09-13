use argon2::password_hash::SaltString;
use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Block, block};
use crate::dtos::block::{BlockAddRequest, BlockResponse};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn add_block(
  req: BlockAddRequest
) -> AppResult<BlockResponse> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let salt = SaltString::generate(rand::thread_rng());
  let model = block::ActiveModel {
    blocker_id: Set(req.blocker_id.into()),
    target_id: Set(req.target_id.into()),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    deleted_at: Set(None),
  };
  let user = Block::insert(model.clone()).exec(db).await?;
  Ok(BlockResponse {
    blocker_id: user.last_insert_id.0,
    target_id: user.last_insert_id.1,
    ..BlockResponse::from(model)
  })
}

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
