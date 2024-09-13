use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::entities::{prelude::Person, person};
use crate::dtos::person::PersonResponse;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use uuid::Uuid;

pub async fn delete_person(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Person::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("person"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
      let person = Person::find_by_id(id).one(db).await?;
      if person.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("person"))).into());
      }

      let mut person: person::ActiveModel = person.unwrap().into();
      person.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(chrono::Local::now())));
      person.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_person(
  paginator_option: Option<PaginatorOption>
) -> AppResult<Vec<PersonResponse>> {
  let db = DB.get().ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let persons = Person::find().paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page).await?;
      let res = persons.into_iter()
        .map(|person: person::Model| PersonResponse::from(person)).collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let persons = Person::find().all(db).await?;
      let res = persons.into_iter()
        .map(|person: person::Model| PersonResponse::from(person)).collect::<Vec<_>>();
      Ok(res)
    }
  }
}
