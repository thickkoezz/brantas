use sea_orm::EntityTrait;
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::VerificationToken;

pub async fn delete_verification_token(
  identifier: String,
  token: String,
) -> AppResult<()> {
  let db = DB.get().ok_or(
    anyhow::anyhow!(t!("database_connection_failed")
    ))?;
  VerificationToken::delete_by_id((identifier, token)).exec(db).await?;
  Ok(())
}
