use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::prelude::VerificationToken;
use sea_orm::EntityTrait;

pub async fn delete_verification_token(identifier: String, token: String) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = VerificationToken::delete_by_id((identifier, token))
    .exec(db)
    .await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("verification_token"))).into()),
    _ => Ok(()),
  }
}
