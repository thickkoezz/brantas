use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::VerificationToken, verification_token};
use crate::dtos::verification_token::VerificationTokenResponse;
use crate::services::PaginatorOption;
use sea_orm::{EntityTrait, PaginatorTrait};

pub async fn delete_verification_token(
  identifier: String,
  token: String,
) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let result = VerificationToken::delete_by_id((identifier, token)).exec(db).await?;
  match result.rows_affected {
    0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("verification_token"))).into()),
    _ => Ok(()),
  }
}

pub async fn get_verification_token(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<VerificationTokenResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let verification_tokens = VerificationToken::find()
        .paginate(db, paginator_option.page_size).fetch_page(paginator_option.page).await?;
      let res = verification_tokens.into_iter()
        .map(|verification_token: verification_token::Model|
          VerificationTokenResponse::from(verification_token)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let verification_tokens = VerificationToken::find().all(db).await?;
      let res = verification_tokens.into_iter()
        .map(|verification_token: verification_token::Model|
          VerificationTokenResponse::from(verification_token)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
