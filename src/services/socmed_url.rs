use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::SocmedUrl;
use crate::entities::socmed_url;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_socmed_url(
  deletion_mode: DeletionMode,
  socmed_url: String,
  owner_id: Uuid,
) -> AppResult<()> {
  match deletion_mode {
    DeletionMode::Hard => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let result = SocmedUrl::delete_by_id((socmed_url, owner_id))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("socmed_url"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let socmed_url = SocmedUrl::find_by_id((
        socmed_url,
        owner_id,
      )).one(db).await?;
      if socmed_url.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("socmed_url"))).into());
      }

      let mut socmed_url: socmed_url::ActiveModel = socmed_url.unwrap().into();
      socmed_url.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      socmed_url.update(db).await?;
      Ok(())
    }
  }
}
