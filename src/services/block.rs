use super::{AddOne, DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::get_db;
use crate::dtos::block::{AddRequest, Response};
use crate::entities::{block, prelude::Block};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{
  ActiveModelTrait, DatabaseConnection, EntityTrait, InsertResult, PaginatorTrait, Set,
};
use uuid::Uuid;

impl AddOne for AddRequest {
  type Output = Response;

  async fn add_one(&self) -> AppResult<Self::Output> {
    let db: &DatabaseConnection = get_db()?;
    let model: Option<block::Model> = Block::find_by_id((self.blocker_id, self.target_id))
      .one(db)
      .await?;
    match model {
      Some(model) => {
        let mut model: block::ActiveModel = model.into();
        model.created_at = Set(DateTimeWithTimeZone::from(chrono::Local::now()));
        model.deleted_at = Set(None);
        let model: block::Model = model.update(db).await?;
        Ok(Response {
          ..Response::from(model)
        })
      },
      None => {
        let model = block::ActiveModel {
          blocker_id: Set(self.blocker_id),
          target_id: Set(self.target_id),
          created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
          deleted_at: Set(None),
        };
        let result: InsertResult<block::ActiveModel> =
          Block::insert(model.clone()).exec(db).await?;
        Ok(Response {
          blocker_id: result.last_insert_id.0,
          target_id: result.last_insert_id.1,
          ..Response::from(model)
        })
      },
    }
  }
}

pub async fn delete_one_by_id_with_mode(
  deletion_mode: DeletionMode,
  id: (Uuid, Uuid),
) -> AppResult<()> {
  let db: &DatabaseConnection = get_db()?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Block::delete_by_id((id.0, id.1)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("block"))).into()),
        _ => Ok(()),
      }
    },
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
        },
        None => Err(anyhow::anyhow!(t!("x_not_exist", x = t!("block"))).into()),
      }
    },
  }
}

pub async fn get_one(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<Response>> {
  let db: &DatabaseConnection = get_db()?;
  let list: Vec<block::Model>;
  match paginator_option {
    Some(paginator_option) => {
      list = Block::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?
    },
    None => list = Block::find().all(db).await?,
  }
  let res = list
    .into_iter()
    .map(|x| Response::from(x))
    .collect::<Vec<_>>();
  Ok(res)
}
