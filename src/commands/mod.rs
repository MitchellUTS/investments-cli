use clap::Subcommand;

pub mod prices;

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(subcommand)]
    Prices(prices::PriceCommands),
}

impl Command {
    pub async fn execute(self, db: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Prices(sub_command) => sub_command.execute(db).await,
        }
    }
}