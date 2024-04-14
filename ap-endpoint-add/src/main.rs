use std::collections::HashMap;

use ap_model::Item;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde_json::Value;

async fn function_handler(
    event: Request,
    db_client: &DynamoDbClient,
) -> Result<Response<Body>, Error> {
    let body_str: String = match event.body() {
        Body::Text(s) => s.to_owned(),
        _ => return Err("Body was not recognized as text or was empty.".into()),
    };

    let Ok(Value::Object(map)) = serde_json::from_str(&body_str) else {
        return Err("Not an object.".into());
    };

    let Ok(_) = serde_json::from_str::<Item>(&body_str) else {
        return Err("Invalid or missing JSON properties.".into());
    };

    let attributes: HashMap<String, AttributeValue> = map
        .iter()
        .map(|element| {
            (
                element.0.to_owned(),
                AttributeValue::S(element.1.to_string()),
            )
        })
        .collect();

    db_client
        .put_item()
        .set_table_name(Some("products".to_owned()))
        .set_item(Some(attributes))
        .send()
        .await
        .map_err(|e| -> aws_sdk_dynamodb::Error {
            tracing::error!("Couldn't send request to DynamoDB: {:?}", e);
            e.into()
        })?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Empty)
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let config = aws_config::from_env().load().await;
    let db_client = DynamoDbClient::new(&config);

    run(service_fn(|event| function_handler(event, &db_client))).await
}
