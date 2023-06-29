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
                    .col(ColumnDef::new(WellDiscovery::WellUid).string())
                    .col(ColumnDef::new(WellDiscovery::WellName).string())
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
