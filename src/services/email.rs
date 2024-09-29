use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::email::EmailDTO;
use crate::entities::{email, prelude::Email};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

pub async fn add_email(req: EmailDTO) -> AppResult<EmailDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = email::ActiveModel {
    email: Set(req.email),
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    is_verified: Set(false),
    is_suspended: Set(false),
  };
  let email = Email::insert(model.clone()).exec(db).await?;
  Ok(EmailDTO {
    email: email.last_insert_id,
    ..EmailDTO::from(model)
  })
}

pub async fn delete_email(deletion_mode: DeletionMode, email: String) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Email::delete_by_id(email).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("email"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let email = Email::find_by_id(email).one(db).await?;
      if email.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("email"))).into());
      }

      let mut email: email::ActiveModel = email.unwrap().into();
      email.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      email.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_email(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<EmailDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let emails = Email::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = emails
        .into_iter()
        .map(|email: email::Model| EmailDTO::from(email))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let emails = Email::find().all(db).await?;
      let res = emails
        .into_iter()
        .map(|email: email::Model| EmailDTO::from(email))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
