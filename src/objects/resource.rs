use crate::objects::price::{EndOfDayRecord, eod_date_format};

use crate::objects::price::Price;

#[derive(Debug, Clone)]
pub struct Resource {
    exchange: String,
    symbol: String,
}

pub fn parse_resource_str(resource_str: &str) -> Result<Resource, String> {
    if let Some((resource, exchange)) = resource_str.split_once('.') {
        Ok(Resource::new(exchange, resource))
    } else {
        Err(format!("Invalid resource string: {}", resource_str))
    }
}


impl Resource {
    pub fn new<StringLike: Into<String>>(exchange: StringLike, symbol: StringLike) -> Self {
        Self {
            exchange: exchange.into(),
            symbol: symbol.into(),
        }
    }

    pub fn id(&self) -> String {
        format!("{symbol}.{exchange}", exchange = self.exchange,  symbol = self.symbol)
    }

    pub fn exchange(&self) -> &String {
        &self.exchange
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub async fn fetch_prices(&self, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) ->  reqwest::Result<Vec<Price>> {
        let api_token = std::env::var("API_TOKEN").expect("API_TOKEN environment variable not set");

        let mut query_params: Vec<(&str, String)> = vec![
            ("fmt", String::from("json")),
            ("period", String::from("d")), // Days
            ("order", String::from("a")), // Ascending
            ("api_token", api_token),
        ];

        match start_date { Some(date) => query_params.push(("from", eod_date_format::to_string(&date))), None => () };
        match end_date   { Some(date) => query_params.push(("to",   eod_date_format::to_string(&date))), None => () };

        let url = format!("https://eodhistoricaldata.com/api/eod/{symbol}.{exchange}", symbol = self.symbol, exchange = self.exchange);
        let url = reqwest::Url::parse_with_params(&url, &query_params).expect("An error occured when building the url");

        let response = reqwest::get(url).await?;
        let data = response.json::<Vec<EndOfDayRecord>>().await?;

        let prices: Vec<Price> = data.iter().map(|record| Price::from_EODR(&self, record)).collect();

        Ok(prices)
    }

    pub async fn prices(&self, db: &sqlx::PgPool) -> sqlx::Result<Vec<Price>> {
        let prices = sqlx::query!(
            r#"
            SELECT resource_id, date, value FROM prices
            WHERE resource_id = $1
            "#,
            self.id()
        ).fetch_all(db).await?;

        let prices = prices.into_iter().map(|price| {
            let resource = parse_resource_str(&price.resource_id).unwrap();
            Price::new(resource, price.date, price.value)
        }).collect();
        
        Ok(prices)
    }
}
