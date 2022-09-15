use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
struct Cli {
    #[clap(short = 'a', long = "abc")]
    abc: Option<String>,

    #[clap(short = 'p', long = "path", parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

#[derive(Debug, Deserialize)]
struct SnapShot {
    id: usize,
    resourceId: usize,
    date: chrono::DateTime<chrono::Utc>,
    price: f32,
    quantity: usize,
    cost: f32,
    payments: f32,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, world! {:?}, {:?}", args.abc, args.path);
    let resp = reqwest::blocking::get("http://localhost:3000/api/v1/snapshots/2").unwrap();
    let snapshots = resp.json::<Vec<SnapShot>>().unwrap();
    for snap in snapshots {
        println!("{:?} {:?}", snap.date, snap.price);
    }
}
