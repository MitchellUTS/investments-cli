use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Resource {
    exchange: String,
    symbol: String,
}
impl Resource {
    pub fn new<StringLike: Into<String>>(exchange: StringLike, symbol: StringLike) -> Self {
        Self {
            exchange: exchange.into(),
            symbol: symbol.into(),
        }
    }

    pub fn fetch_prices(self, api_key: &str, start_date: Option<chrono::NaiveDate>, end_date: Option<chrono::NaiveDate>) ->  reqwest::Result<Vec<EndOfDayRecord>> {
        let mut query_params: Vec<(&str, String)> = vec![
            ("fmt", String::from("json")),
            ("period", String::from("d")), // Days
            ("order", String::from("a")), // Ascending
            ("api_token", String::from(api_key)),
        ];

        match start_date { Some(date) => query_params.push(("from", date.format(eod_date_format::FORMAT).to_string())), None => () };
        match end_date   { Some(date) => query_params.push(("to",   date.format(eod_date_format::FORMAT).to_string())), None => () };

        let url = format!("https://eodhistoricaldata.com/api/eod/{symbol}.{exchange}", 
            symbol = self.symbol,
            exchange = self.exchange
        );

        let url = reqwest::Url::parse_with_params(&url, &query_params).expect("An error occured when building the url");

        let response = reqwest::blocking::get(url)?;
        let data = response.json::<Vec<EndOfDayRecord>>()?;

        Ok(data)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfDayRecord {
    #[serde(with = "eod_date_format")]
    date: chrono::NaiveDate,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    adjusted_close: f32,
    volume: usize,
}

mod eod_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub(crate) const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}