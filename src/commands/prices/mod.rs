pub mod fetch;

#[derive(Debug, clap::Subcommand)]
pub enum PriceCommands {
    Fetch(fetch::PriceFetchArgs),
}

impl PriceCommands {
    pub async fn execute(self, db: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            PriceCommands::Fetch(args) => args.execute(db).await,
        }
    }
}