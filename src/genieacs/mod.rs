use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::utils::url::encode;

pub mod device;

pub struct GenieACS {
    pub url: String,
}

impl GenieACS {
    pub fn new(url: &str) -> Self {
        if url.ends_with('/') {
            url.to_string().pop();
        }

        Self {
            url: url.to_string(),
        }
    }

    pub async fn find_device(&self, param: &str, value: &str) {
        let query = encode(
            json!({
              param: value
            })
            .to_string(),
        );

        dbg!(&query);

        let response_result = reqwest::get(format!("{}devices/?query={}", self.url, query)).await;

        match response_result {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    let data = response.json::<Value>().await.unwrap();

                    match data {
                        Value::Array(values) if !values.is_empty() => {
                            let keys = device::parser::read_all_keys(values.first().unwrap());

                            for key in &keys {
                                println!("{:?}", key);
                            }

                            println!("{}", keys.len());
                        }
                        _ => todo!(),
                    }
                }
                StatusCode::NOT_FOUND => {}
                _ => {}
            },
            Err(_) => todo!(),
        }
    }
}
