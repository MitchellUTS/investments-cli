use crate::objects::resource::Resource;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Price {
    resource: Resource,
    date: chrono::NaiveDate,
    value: f32,
}

impl Price {
    pub fn new(resource: Resource, date: chrono::NaiveDate, value: f32) -> Self {
        Self {
            resource,
            date,
            value,
        }
    }
    
    pub fn resource(&self) -> &Resource {
        &self.resource
    }
    
    pub fn date(&self) -> &chrono::NaiveDate {
        &self.date
    }
    
    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn from_EODR(resource: &Resource, eodr: &EndOfDayRecord) -> Self {
        Self::new(
            resource.clone(),
            eodr.date,
            eodr.close,
        )
    }
    
    pub async fn save(&self, db: &sqlx::PgPool) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO prices (resource_id, date, value)
            VALUES ($1, $2, $3)
            ON CONFLICT (resource_id, date) DO UPDATE SET value = $3
            "#,
            self.resource.id(),
            self.date,
            self.value,
        ).execute(db).await?;
        
        Ok(())
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

pub fn parse_date_str(date_str: &str) -> Result<chrono::NaiveDate, String> {
    eod_date_format::from_str(date_str).map_err(|err| err.to_string())
}

pub mod eod_date_format {
    use chrono::{NaiveDate, ParseResult};
    use serde::{self, Deserialize, Serializer, Deserializer};
    
    pub(crate) const FORMAT: &'static str = "%Y-%m-%d";
    
    pub fn from_str(date_str: &str) -> ParseResult<NaiveDate> {
        NaiveDate::parse_from_str(date_str, FORMAT)
    }
    pub fn to_string(date: &NaiveDate) -> String {
        date.format(FORMAT).to_string()
    }
    
    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", to_string(date));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        from_str(&s)
            .map_err(serde::de::Error::custom)
    }
}