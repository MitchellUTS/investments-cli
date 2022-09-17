use clap::{Parser, Subcommand};

use investments_cli::objects::resource::{*};

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

fn parse_date_str(date_str: &str) -> Result<chrono::NaiveDate, String> {
    eod_date_format::from_str(date_str).map_err(|err| err.to_string())
}

#[derive(Debug, Subcommand)]
enum PriceCommands {
    Fetch { 
        #[clap(value_parser(clap::builder::ValueParser::new(parse_resource_str)))]
        resources: Vec<Resource>,

        // resource: Resource,
        // api_key: String,

        #[clap(short = 's', long = "start-date", value_parser(clap::builder::ValueParser::new(parse_date_str)))]
        start_date: Option<chrono::NaiveDate>,
        #[clap(short = 'e', long = "end-date", value_parser(clap::builder::ValueParser::new(parse_date_str)))]
        end_date: Option<chrono::NaiveDate>,
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



const DEMO_API_KEY: &'static str = "OeAFFmMliFG5orCUuwAKQ8l4WWFQ67YX";
const DEFAULT_RESOURCES: [&'static str; 1] = ["MCD.US"];
// const DEFAULT_RESOURCES: Vec<Resource> = DEFAULT_RESOURCED_STR.iter().map(|s| parse_resource_str(s).unwrap()).collect();

fn main() {
    let args = Cli::parse();
    // println!("Args: {:?}", args);

    match args.command {
        Some(Command::Prices(PriceCommands::Fetch { mut resources, start_date, end_date })) => {
            //Some(chrono::NaiveDate::from_ymd(2022, 01, 01))
            if resources.is_empty() {
                for resource_str in DEFAULT_RESOURCES {
                    resources.push(parse_resource_str(resource_str).unwrap())
                }
            }

            for resource in resources {
                let prices = resource.fetch_prices(DEMO_API_KEY, start_date, end_date).unwrap();
                let max_price = prices.iter().max_by(|a, b| a.cmp_price(&b)).unwrap();
                println!("Max EOD Price: {:?}", max_price);
            }
            
            // // Sort by date ascending
            // data.sort_by(|a, b| a.cmp_date(&b));
            // println!("{:?}", data.first());
            // println!("{:?}", data.last());
            
            // Find max price
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
