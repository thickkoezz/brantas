use crate::entities::balance_history::{ActiveModel, Model};
use salvo::macros::Extractible;
use salvo::oapi::ToSchema;
use sea_orm::prelude::{DateTimeWithTimeZone, Decimal};
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type ID = (Uuid, DateTimeWithTimeZone, Uuid);

#[derive(Debug, Default, Deserialize, Serialize, Extractible, ToSchema, Validate)]
pub struct BalanceHistoryDTO {
  pub owner_id: Uuid,
  pub created_at: DateTimeWithTimeZone,
  pub ref_id: Uuid,
  pub amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl BalanceHistoryDTO {
  pub fn new(owner_id: Uuid, ref_id: Uuid) -> Self {
    Self {
      owner_id,
      created_at: DateTimeWithTimeZone::from(chrono::Local::now()),
      ref_id,
      ..Default::default()
    }
  }

  pub fn create(owner_id: Uuid, ref_id: Uuid, amount: Decimal) -> Self {
    Self {
      amount,
      ..Self::new(owner_id, ref_id)
    }
  }

  pub fn delete(&mut self) -> &mut Self {
    self.deleted_at = Option::from(DateTimeWithTimeZone::from(chrono::Local::now()));
    self
  }

  pub fn get_id(&self) -> ID {
    (
      self.owner_id.clone(),
      self.created_at.clone(),
      self.ref_id.clone(),
    )
  }

  pub fn set_id(&mut self, v: ID) -> &mut Self {
    self.owner_id = v.0;
    self.created_at = v.1;
    self.ref_id = v.2;
    self
  }

  pub fn set_owner_id(&mut self, v: Uuid) -> &mut Self {
    self.owner_id = v;
    self
  }

  pub fn set_created_at(&mut self, v: DateTimeWithTimeZone) -> &mut Self {
    self.created_at = v;
    self
  }

  pub fn set_ref_id(&mut self, v: Uuid) -> &mut Self {
    self.ref_id = v;
    self
  }

  pub fn set_amount(&mut self, v: Decimal) -> &mut Self {
    self.amount = v;
    self
  }

  pub fn set_deleted_at(&mut self, v: Option<DateTimeWithTimeZone>) -> &mut Self {
    self.deleted_at = v;
    self
  }
}

impl From<Model> for BalanceHistoryDTO {
  fn from(m: Model) -> Self {
    Self {
      owner_id: m.owner_id,
      created_at: m.created_at,
      ref_id: m.ref_id,
      amount: m.amount,
      deleted_at: m.deleted_at,
    }
  }
}

impl From<ActiveModel> for BalanceHistoryDTO {
  fn from(m: ActiveModel) -> Self {
    Self {
      owner_id: m.owner_id.unwrap(),
      created_at: m.created_at.unwrap(),
      ref_id: m.ref_id.unwrap(),
      amount: m.amount.unwrap(),
      deleted_at: m.deleted_at.unwrap(),
    }
  }
}
