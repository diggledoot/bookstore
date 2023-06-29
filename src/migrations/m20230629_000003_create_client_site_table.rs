use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230629_000003_create_client_site_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClientSite::Table)
                    .col(
                        ColumnDef::new(ClientSite::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ClientSite::IndexWellUrl).string())
                    .col(ColumnDef::new(ClientSite::Username).string())
                    .col(ColumnDef::new(ClientSite::Password).string())
                    .col(ColumnDef::new(ClientSite::IsIndexed).boolean())
                    .col(ColumnDef::new(ClientSite::IsIndexing).boolean())
                    .col(ColumnDef::new(ClientSite::LastIndexedAt).date_time())
                    .col(ColumnDef::new(ClientSite::TotalToIndex).integer())
                    .col(ColumnDef::new(ClientSite::TotalIndexed).integer())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClientSite::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ClientSite {
    Table,
    Id,
    IndexWellUrl,
    Username,
    Password,
    IsIndexed,
    IsIndexing,
    LastIndexedAt,
    TotalToIndex,
    TotalIndexed,
}
