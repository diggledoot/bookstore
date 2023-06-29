use sea_orm_migration::prelude::*;
pub struct Migrator;

use super::m20230629_000001_create_well_discovery_config_table;
use super::m20230629_000002_create_well_discovery_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230629_000001_create_well_discovery_config_table::Migration),
            Box::new(m20230629_000002_create_well_discovery_table::Migration),
        ]
    }
}
