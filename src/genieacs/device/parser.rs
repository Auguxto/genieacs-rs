use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug)]
pub enum DeviceValue {
    Text(String),
    Number(isize),
    Boolean(bool),
}

pub fn read_keys(json_value: &Value, prefix: String, keys: &mut HashMap<String, DeviceValue>) {
    match json_value {
        Value::Object(map) => {
            for (key, value) in map {
                let full_key = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                match value {
                    Value::Number(_) => keys.insert(
                        full_key.clone(),
                        DeviceValue::Number(value.to_string().parse::<isize>().unwrap()),
                    ),
                    Value::String(_) => {
                        keys.insert(full_key.clone(), DeviceValue::Text(value.to_string()))
                    }
                    Value::Bool(_) => keys.insert(
                        full_key.clone(),
                        DeviceValue::Boolean(value.as_bool().unwrap()),
                    ),
                    _ => None,
                };
                read_keys(value, full_key, keys);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                let full_key = format!("{}[{}]", prefix, index);
                read_keys(value, full_key, keys);
            }
        }
        _ => {}
    }
}

pub fn read_all_keys(json_value: &Value) -> HashMap<String, DeviceValue> {
    let mut keys: HashMap<String, DeviceValue> = HashMap::new();
    read_keys(json_value, "".to_string(), &mut keys);

    keys
}

pub fn parse_raw_device(data: &Vec<Value>) {
    println!("{:?}", data);
}
