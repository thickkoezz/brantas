use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::link_code::{LinkCodeAddRequest, LinkCodeResponse};
use crate::entities::{link_code, prelude::LinkCode};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

pub async fn add_link_code(req: LinkCodeAddRequest) -> AppResult<LinkCodeResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = link_code::ActiveModel {
    link_id: Set(req.link_id),
    code: if req.is_code_manually_typed {
      Set(req.code)
    } else {
      Default::default()
    },
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    expired_at: Set(req.expired_at),
    is_public: Set(req.is_public),
    is_code_manually_typed: Set(req.is_code_manually_typed),
    deleted_at: Set(None),
  };
  let link_code = LinkCode::insert(model.clone()).exec(db).await?;
  Ok(LinkCodeResponse {
    code: link_code.last_insert_id,
    ..LinkCodeResponse::from(model)
  })
}

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
      let link_code = LinkCode::find_by_id(code).one(db).await?;
      if link_code.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("code"))).into());
      }

      let mut link_code: link_code::ActiveModel = link_code.unwrap().into();
      link_code.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      link_code.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_link_code(
  paginator_option: Option<PaginatorOption>,
) -> AppResult<Vec<LinkCodeResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let link_codes = LinkCode::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = link_codes
        .into_iter()
        .map(|link_code: link_code::Model| LinkCodeResponse::from(link_code))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let link_codes = LinkCode::find().all(db).await?;
      let res = link_codes
        .into_iter()
        .map(|link_code: link_code::Model| LinkCodeResponse::from(link_code))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
