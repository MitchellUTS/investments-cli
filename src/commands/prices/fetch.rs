use clap::{Parser};

use futures::future::try_join_all;
use investments_cli::objects::{resource::{Resource, parse_resource_str}, price::parse_date_str};

#[derive(Debug, Parser)]
pub struct PriceFetchArgs {
    #[clap(value_parser(clap::builder::ValueParser::new(parse_resource_str)))]
    resources: Vec<Resource>,

    #[clap(short = 's', long = "start-date", value_parser(clap::builder::ValueParser::new(parse_date_str)))]
    start_date: Option<chrono::NaiveDate>,

    #[clap(short = 'e', long = "end-date", value_parser(clap::builder::ValueParser::new(parse_date_str)))]
    end_date: Option<chrono::NaiveDate>,
}

const DEFAULT_RESOURCES: [&'static str; 1] = ["MCD.US"];

impl PriceFetchArgs {
    pub async fn execute(mut self, db: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
        //Some(chrono::NaiveDate::from_ymd(2022, 01, 01))

        if self.resources.is_empty() {
            for resource_str in DEFAULT_RESOURCES {
                self.resources.push(parse_resource_str(resource_str).unwrap())
            }
        }
    
        for resource in self.resources {
            let prices = resource.fetch_prices(self.start_date, self.end_date).await?;
            try_join_all(prices.iter().map(|price| price.save(db))).await?;
            println!("Saved {} price(s) to the database", prices.len());
        }
    
        Ok(())
    }
}