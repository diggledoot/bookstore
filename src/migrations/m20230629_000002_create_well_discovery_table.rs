use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230629_000002_create_well_discovery_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WellDiscovery::Table)
                    .col(
                        ColumnDef::new(WellDiscovery::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WellDiscovery::ClientSiteId).string())
                    .col(ColumnDef::new(WellDiscovery::WellUid).string())
                    .col(ColumnDef::new(WellDiscovery::WellName).string())
                    .col(ColumnDef::new(WellDiscovery::WellTimeZone).integer())
                    .col(ColumnDef::new(WellDiscovery::WellboreUid).string())
                    .col(ColumnDef::new(WellDiscovery::WellboreName).string())
                    .col(ColumnDef::new(WellDiscovery::LogUid).string())
                    .col(ColumnDef::new(WellDiscovery::LogName).string())
                    .col(ColumnDef::new(WellDiscovery::IndexType).string())
                    .col(ColumnDef::new(WellDiscovery::StartDateTimeIndex).date_time())
                    .col(ColumnDef::new(WellDiscovery::RigUid).string())
                    .col(ColumnDef::new(WellDiscovery::RigName).string())
                    .col(ColumnDef::new(WellDiscovery::Status).string())
                    .col(ColumnDef::new(WellDiscovery::IndexedOperationUid).string())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WellDiscovery::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum WellDiscovery {
    Table,
    Id,
    ClientSiteId,
    WellUid,
    WellName,
    WellTimeZone,
    WellboreUid,
    WellboreName,
    LogUid,
    LogName,
    IndexType,
    StartDateTimeIndex,
    RigUid,
    RigName,
    Status,
    IndexedOperationUid,
}
