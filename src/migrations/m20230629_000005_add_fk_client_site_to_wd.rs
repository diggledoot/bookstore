use sea_orm_migration::prelude::*;

use super::{
    m20230629_000002_create_well_discovery_table::WellDiscovery,
    m20230629_000003_create_client_site_table::ClientSite,
};
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230629_000005_add_fk_client_site_to_wd"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("FK_client_site_wd")
                    .from(WellDiscovery::Table, WellDiscovery::ClientSiteId)
                    .to(ClientSite::Table, ClientSite::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("FK_client_site_wd")
                    .table(WellDiscovery::Table)
                    .to_owned(),
            )
            .await
    }
}
