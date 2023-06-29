mod migrator;

use std::error::Error;

use migrator::migrator::Migrator;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_migrations().await?;
    Ok(())
}

async fn run_migrations() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;
    let db = Database::connect(&database_url).await?;
    Migrator::refresh(&db).await?;
    println!("Migrations ran!");
    Ok(())
}
