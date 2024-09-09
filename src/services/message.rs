use super::DeletionMode;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::Message;
use crate::entities::message;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_message(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  receiver_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Message::delete_by_id((
        owner_id,
        receiver_id,
        created_at,
      ))
        .exec(db)
        .await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("message"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let db = DB
        .get()
        .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let message = Message::find_by_id((
        owner_id,
        receiver_id,
        created_at,
      ))
        .one(db)
        .await?;
      if message.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("message"))).into());
      }

      let mut message: message::ActiveModel = message.unwrap().into();
      message.deleted_at = Set(Option::from(
        DateTimeWithTimeZone::from(chrono::Local::now())
      ));
      message.update(db).await?;
      Ok(())
    }
  }
}
