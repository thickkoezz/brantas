use crate::entities::block::{ActiveModel, Model};
use salvo::oapi::ToSchema;
use salvo::prelude::Extractible;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use validator::{Validate, ValidationError};

pub type ID = (Uuid, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct BlockDTO {
  pub blocker_id: Uuid,
  pub target_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl BlockDTO {
  pub fn new(blocker_id: Uuid, target_id: Uuid) -> Self {
    Self {
      blocker_id,
      target_id,
      created_at: DateTimeWithTimeZone::from(Local::now()),
      ..Default::default()
    }
  }

  pub fn create(blocker_id: Uuid, target_id: Uuid) -> Self {
    Self {
      ..Self::new(blocker_id, target_id)
    }
  }

  pub fn validate(&self) -> Result<(), ValidationError> {
    if self.blocker_id == self.target_id {
      return Err(
        ValidationError::new("block.self").with_message(Cow::from(
          anyhow::anyhow!(t!("unable_to_block_self")).into(),
        )),
      );
    }
    Ok(())
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (self.blocker_id.clone(), self.target_id.clone())
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.blocker_id = v.0;
    self.target_id = v.1;
    self
  }

  pub fn set_blocker_id(&mut self, v: Uuid) -> &mut Self {
    self.blocker_id = v;
    self
  }

  pub fn set_target_id(&mut self, v: Uuid) -> &mut Self {
    self.target_id = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }
}

impl From<Model> for BlockDTO {
  fn from(m: Model) -> Self {
    Self {
      blocker_id: m.blocker_id,
      target_id: m.target_id,
      created_at: m.created_at,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for BlockDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      blocker_id: m.blocker_id.unwrap(),
      target_id: m.target_id.unwrap(),
      created_at: m.created_at.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}

impl Into<Model> for BlockDTO {
  fn into(self) -> Model {
    Model {
      blocker_id: self.blocker_id,
      target_id: self.target_id,
      created_at: self.created_at,
      deleted_at: self.deleted_at,
    }
  }
}

impl Into<ActiveModel> for BlockDTO {
  fn into(self) -> ActiveModel {
    ActiveModel {
      blocker_id: Set(self.blocker_id),
      target_id: Set(self.target_id),
      created_at: Set(self.created_at),
      deleted_at: Set(self.deleted_at),
    }
  }
}
