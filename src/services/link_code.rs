use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::LinkCode;
use crate::entities::link_code;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;

pub async fn delete_link_code(deletion_mode: DeletionMode, code: String) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = LinkCode::delete_by_id(code).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("code"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let link_code = LinkCode::find_by_id(code).one(db).await?;
      if link_code.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("code"))).into());
      }

      let mut link_code: link_code::ActiveModel = link_code.unwrap().into();
      link_code.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      link_code.update(db).await?;
      Ok(())
    },
  }
}
