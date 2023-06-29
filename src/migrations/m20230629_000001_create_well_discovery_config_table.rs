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
}
