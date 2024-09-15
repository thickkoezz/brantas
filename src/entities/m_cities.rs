//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "m_cities")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: i32,
  pub name: String,
  pub state_id: i16,
  pub state_code: String,
  pub country_id: i16,
  pub country_code: String,
  #[sea_orm(column_type = "Decimal(Some((10, 8)))")]
  pub latitude: Decimal,
  #[sea_orm(column_type = "Decimal(Some((11, 8)))")]
  pub longitude: Decimal,
  pub created_at: Option<DateTime>,
  pub updated_at: Option<DateTime>,
  pub flag: i16,
  pub wikidataid: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::m_countries::Entity",
    from = "Column::CountryId",
    to = "super::m_countries::Column::Id",
    on_update = "Restrict",
    on_delete = "Restrict"
  )]
  MCountries,
  #[sea_orm(
    belongs_to = "super::m_states::Entity",
    from = "Column::StateId",
    to = "super::m_states::Column::Id",
    on_update = "Restrict",
    on_delete = "Restrict"
  )]
  MStates,
  #[sea_orm(has_many = "super::organization_address::Entity")]
  OrganizationAddress,
}

impl Related<super::m_countries::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::MCountries.def()
  }
}

impl Related<super::m_states::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::MStates.def()
  }
}

impl Related<super::organization_address::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::OrganizationAddress.def()
  }
}

impl Related<super::organization::Entity> for Entity {
  fn to() -> RelationDef {
    super::organization_address::Relation::Organization.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::organization_address::Relation::MCities.def().rev())
  }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
  #[sea_orm(entity = "super::m_countries::Entity")]
  MCountries,
  #[sea_orm(entity = "super::m_states::Entity")]
  MStates,
  #[sea_orm(entity = "super::organization_address::Entity")]
  OrganizationAddress,
  #[sea_orm(entity = "super::organization::Entity")]
  Organization,
}
