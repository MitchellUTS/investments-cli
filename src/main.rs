use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

use investments_cli::objects::resource::{*, self};

#[derive(Debug, Parser)]
struct Cli {

    #[clap(short = 'a', long = "abc")]
    abc: Option<String>,

    // #[clap(short = 'p', long = "path", parse(from_os_str))]
    // path: Option<std::path::PathBuf>,

    #[clap(subcommand)]
    command: Option<Command>,

    #[clap(short = 'v', long = "verbose")]
    verbose: Option<bool>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(subcommand)]
    Prices (PriceCommands),
}

fn parse_resource_str(resource_str: &str) -> Result<Resource, String> {
    if let Some((resource, exchange)) = resource_str.split_once('.') {
        Ok(Resource::new(exchange, resource))
    } else {
        Err(format!("Invalid resource string: {}", resource_str))
    }
}

#[derive(Debug, Subcommand)]
enum PriceCommands {
    Fetch { 
        #[clap(value_parser(clap::builder::ValueParser::new(parse_resource_str)))]
        resource: Resource,
        // resource: Resource,
        // api_key: String,
        // start_date: Option<chrono::NaiveDate>,
        // end_date: Option<chrono::NaiveDate>,
    },
}

// #[derive(Debug, Deserialize)]
// struct SnapShot {
//     id: usize,
//     resourceId: usize,
//     date: chrono::DateTime<chrono::Utc>,
//     price: f32,
//     quantity: usize,
//     cost: f32,
//     payments: f32,
// }





fn main() {
    let args = Cli::parse();
    // println!("Args: {:?}", args);

    match args.command {
        Some(Command::Prices(PriceCommands::Fetch { resource })) => {
            let data = resource.fetch_prices("OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX", Some(chrono::NaiveDate::from_ymd(2022, 01, 01)), None).unwrap();
            println!("Data: {:?}", data.len());
        },
        None => {
            println!("No command");
        }
    }

    // let resp = reqwest::blocking::get("http://localhost:3000/api/v1/snapshots/2").unwrap();
    // let snapshots = resp.json::<Vec<SnapShot>>().unwrap();
    // for snap in snapshots {
    //     println!("{:?} {:?}", snap.date, snap.price);
    // }
}
