use chrono::{DateTime, Utc};
use serde::{Serializer, Deserializer};
use serde::{Deserialize, Serialize};
use serde_yaml::to_string as to_yaml;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use toml::to_string as to_toml;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test() {
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();
        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].description, "Gift 1");
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

// Функція серіалізації дати
fn serialize_date<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("Date: {}", date))
}

// Функція десеріалізації дати
fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("Date: ", ""))
}

fn main() {
    let mut file = File::open("request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let request: Request = serde_json::from_str(&json_str).unwrap();
    println!("{:?}", request);

    let yaml_str = to_yaml(&request).unwrap();
    println!("YAML:\n{}", yaml_str);

    let toml_str = to_toml(&request).unwrap();
    println!("TOML:\n{}", toml_str);

    let event = Event {
        name: "Концерт".to_string(),
        date: "2024-11-15".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Помилка серіалізації");
    println!("Серіалізований JSON з кастомною датою: {}", json);

    let deserialized_event: Event = serde_json::from_str(&json).expect("Помилка десеріалізації");
    println!("Десеріалізована подія: {:?}", deserialized_event);
}
