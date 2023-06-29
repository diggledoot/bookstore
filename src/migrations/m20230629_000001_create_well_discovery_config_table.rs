use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230629_000001_create_well_discovery_config_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WellDiscoveryConfig::Table)
                    .col(
                        ColumnDef::new(WellDiscoveryConfig::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WellDiscoveryConfig::ClientSiteId).integer())
                    .col(ColumnDef::new(WellDiscoveryConfig::LogUidWell).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::LogUidWellbore).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::LogUid).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::LogName).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::IndexType).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::ObjectGrowing).boolean())
                    .col(ColumnDef::new(WellDiscoveryConfig::ServiceId).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::ServiceName).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::ApiVersion).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::Endpoint).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::Username).string())
                    .col(ColumnDef::new(WellDiscoveryConfig::Password).string())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WellDiscoveryConfig::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum WellDiscoveryConfig {
    Table,
    Id,
    ClientSiteId,
    LogUidWell,
    LogUidWellbore,
    LogUid,
    LogName,
    IndexType,
    ObjectGrowing,
    ServiceId,
    ServiceName,
    ApiVersion,
    Endpoint,
    Username,
    Password,
}
