use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::person::PersonDTO;
use crate::entities::{person, prelude::Person};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_person(req: PersonDTO) -> AppResult<PersonDTO> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = person::ActiveModel {
    id: Set(Uuid::new_v4()),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    first_name: Set(req.first_name),
    middle_name: Set(req.middle_name),
    last_name: Set(req.last_name),
    dob: Set(req.dob),
    sex: Set(req.sex),
    deceased_at: Set(req.deceased_at),
    extra_info: Set(req.extra_info),
    is_deceased: Set(req.is_deceased),
    photo: Set(req.photo),
    email: Set(req.email),
    is_email_verified: Set(false),
    nickname: Set(req.nickname),
  };
  let person = Person::insert(model.clone()).exec(db).await?;
  Ok(PersonDTO {
    id: person.last_insert_id,
    ..PersonDTO::from(model)
  })
}

pub async fn delete_person(deletion_mode: DeletionMode, id: Uuid) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Person::delete_by_id(id).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("person"))).into()),
        _ => Ok(()),
      }
    }
    DeletionMode::Soft => {
      let person = Person::find_by_id(id).one(db).await?;
      if person.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("person"))).into());
      }

      let mut person: person::ActiveModel = person.unwrap().into();
      person.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      person.update(db).await?;
      Ok(())
    }
  }
}

pub async fn get_person(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<PersonDTO>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let persons = Person::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = persons
        .into_iter()
        .map(|person: person::Model| PersonDTO::from(person))
        .collect::<Vec<_>>();
      Ok(res)
    }
    None => {
      let persons = Person::find().all(db).await?;
      let res = persons
        .into_iter()
        .map(|person: person::Model| PersonDTO::from(person))
        .collect::<Vec<_>>();
      Ok(res)
    }
  }
}
