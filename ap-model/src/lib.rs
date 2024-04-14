use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn as_string(val: Option<&AttributeValue>, default: &String) -> String {
    if let Some(v) = val {
        if let Ok(s) = v.as_s() {
            return s.to_owned();
        }
    }
    default.to_owned()
}

fn as_u32(val: Option<&AttributeValue>, default: u32) -> u32 {
    if let Some(v) = val {
        if let Ok(n) = v.as_n() {
            if let Ok(n) = n.parse::<u32>() {
                return n;
            }
        }
    }
    default
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "ProductId")]
    product_id: u32,
    name: String,
    stock: u32,
}

impl Item {
    pub fn new(product_id: u32, name: String, stock: u32) -> Self {
        Item {
            product_id,
            name,
            stock,
        }
    }
}

impl From<&HashMap<String, AttributeValue>> for Item {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        Item::new(
            as_u32(value.get("product_id"), 0),
            as_string(value.get("name"), &"".to_string()),
            as_u32(value.get("stock"), 0),
        )
    }
}
