//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "person")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub dob: Option<Date>,
    pub sex: Option<bool>,
    pub deceased_at: Option<Date>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub extra_info: Option<Json>,
    pub is_deceased: bool,
    pub photo: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub email: Option<String>,
    pub is_email_verified: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::job::Entity")]
    Job,
    #[sea_orm(has_many = "super::job_project::Entity")]
    JobProject,
    #[sea_orm(has_many = "super::job_skill::Entity")]
    JobSkill,
    #[sea_orm(has_many = "super::organization_administrator::Entity")]
    OrganizationAdministrator,
    #[sea_orm(has_many = "super::project::Entity")]
    Project,
    #[sea_orm(has_many = "super::project_skill::Entity")]
    ProjectSkill,
    #[sea_orm(has_many = "super::skill::Entity")]
    Skill,
    #[sea_orm(has_many = "super::user_account::Entity")]
    UserAccount,
}

impl Related<super::job::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Job.def()
    }
}

impl Related<super::job_project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobProject.def()
    }
}

impl Related<super::job_skill::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobSkill.def()
    }
}

impl Related<super::organization_administrator::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrganizationAdministrator.def()
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl Related<super::project_skill::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectSkill.def()
    }
}

impl Related<super::skill::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Skill.def()
    }
}

impl Related<super::user_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAccount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::job::Entity")]
    Job,
    #[sea_orm(entity = "super::job_project::Entity")]
    JobProject,
    #[sea_orm(entity = "super::job_skill::Entity")]
    JobSkill,
    #[sea_orm(entity = "super::organization_administrator::Entity")]
    OrganizationAdministrator,
    #[sea_orm(entity = "super::project::Entity")]
    Project,
    #[sea_orm(entity = "super::project_skill::Entity")]
    ProjectSkill,
    #[sea_orm(entity = "super::skill::Entity")]
    Skill,
    #[sea_orm(entity = "super::user_account::Entity")]
    UserAccount,
}
