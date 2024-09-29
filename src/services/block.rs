use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::get_db;
use crate::dtos::block::{BlockDTO, ID};
use crate::entities::block::{Entity, Model};
use crate::entities::{block, prelude::Block};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult,
  PaginatorTrait, QueryFilter, QueryOrder, Select, Set,
};
use uuid::Uuid;

pub async fn get_by_id(id: ID) -> AppResult<BlockDTO> {
  let db = get_db()?;
  let item = Block::find_by_id(id).one(db).await?;
  match item {
    Some(item) => Ok(BlockDTO::from(item)),
    None => Err(anyhow::anyhow!(t!("x_not_found", x = t!("block"))).into()),
  }
}

async fn get_list(
  db: &DatabaseConnection,
  select: Select<Entity>,
  paginator_option: Option<PaginatorOption>,
) -> Result<Vec<Model>, DbErr> {
  let list = match paginator_option {
    Some(paginator_option) => {
      select
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await
    }
    None => select.all(db).await,
  };
  list
}

pub async fn get_all(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<BlockDTO>> {
  let db = get_db()?;
  let select = Block::find().order_by_desc(block::Column::CreatedAt);
  let list = get_list(db, select, paginator_option).await?;
  let res = list
    .into_iter()
    .map(|x| BlockDTO::from(x))
    .collect::<Vec<_>>();
  Ok(res)
}

pub async fn get_many_by_blocker_id(
  blocker_id: Uuid,
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<BlockDTO>> {
  let db = get_db()?;
  let select = Block::find()
    .filter(block::Column::BlockerId.eq(blocker_id))
    .order_by_desc(block::Column::CreatedAt);
  let list = get_list(db, select, paginator_option).await?;
  let res = list
    .into_iter()
    .map(|x| BlockDTO::from(x))
    .collect::<Vec<_>>();
  Ok(res)
}

pub async fn get_many_by_target_id(
  target_id: Uuid,
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<BlockDTO>> {
  let db = get_db()?;
  let select = Block::find()
    .filter(block::Column::TargetId.eq(target_id))
    .order_by_desc(block::Column::CreatedAt);
  let list = get_list(db, select, paginator_option).await?;
  let res = list
    .into_iter()
    .map(|x| BlockDTO::from(x))
    .collect::<Vec<_>>();
  Ok(res)
}

async fn add_one(item: BlockDTO) -> AppResult<BlockDTO> {
  let db = get_db()?;
  let model: Option<Model> = Block::find_by_id(item.get_id()).one(db).await?;
  match model {
    Some(model) => {
      let mut model: block::ActiveModel = model.into();
      model.created_at = Set(DateTimeWithTimeZone::from(chrono::Local::now()));
      model.deleted_at = Set(None);
      let model: Model = model.update(db).await?;
      Ok(BlockDTO {
        ..BlockDTO::from(model)
      })
    }
    None => {
      let model = block::ActiveModel {
        blocker_id: Set(item.blocker_id),
        target_id: Set(item.target_id),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
        deleted_at: Set(None),
      };
      let result: InsertResult<block::ActiveModel> = Block::insert(model.clone()).exec(db).await?;
      Ok(BlockDTO {
        blocker_id: result.last_insert_id.0,
        target_id: result.last_insert_id.1,
        ..BlockDTO::from(model)
      })
    }
  }
}

async fn delete_one(id: (Uuid, Uuid), deletion_mode: DeletionMode) -> AppResult<()> {
  let db = get_db()?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Block::delete_by_id((id.0, id.1)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("block"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let model = Block::find_by_id((id.0, id.1)).one(db).await?;
      match model {
        Some(model) => {
          let mut model: block::ActiveModel = model.into();
          model.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
            chrono::Local::now(),
          )));
          model.update(db).await?;
          Ok(())
        }
        None => Err(anyhow::anyhow!(t!("x_not_exist", x = t!("block"))).into()),
      }
    }
  }
}
