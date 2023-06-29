//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "well_discovery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub client_site_id: Option<i32>,
    pub well_uid: Option<String>,
    pub well_name: Option<String>,
    pub well_time_zone: Option<i32>,
    pub wellbore_uid: Option<String>,
    pub wellbore_name: Option<String>,
    pub log_uid: Option<String>,
    pub log_name: Option<String>,
    pub index_type: Option<String>,
    pub start_date_time_index: Option<DateTime>,
    pub rig_uid: Option<String>,
    pub rig_name: Option<String>,
    pub status: Option<String>,
    pub indexed_operation_uid: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::client_site::Entity",
        from = "Column::ClientSiteId",
        to = "super::client_site::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    ClientSite,
}

impl Related<super::client_site::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClientSite.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
