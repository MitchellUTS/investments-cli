use clap::{Parser};
mod commands;

#[derive(Debug, Parser)]
struct Cli {

    #[clap(short = 'a', long = "abc")]
    abc: Option<String>,

    // #[clap(short = 'p', long = "path", parse(from_os_str))]
    // path: Option<std::path::PathBuf>,

    #[clap(subcommand)]
    command: Option<commands::Command>,

    #[clap(short = 'v', long = "verbose")]
    verbose: Option<bool>,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub async fn execute(self, db: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Some(command) => command.execute(db).await,
            None => Err("Unknown command".to_string())?,
        }
    }
}

async fn get_db() -> sqlx::Result<sqlx::PgPool> {
    use sqlx::PgPool;

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let db = PgPool::connect(&url).await?;

    Ok(db)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let db = get_db().await?;
    
    Cli::new().execute(&db).await?;
    
    
    Ok(())
}

