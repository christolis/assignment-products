use ap_model::Item;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Response};

pub async fn get_items(client: &Client) -> Result<Response<Body>, Error> {
    let query_output = client.scan().table_name("products").send().await.map_err(
        |e| -> aws_sdk_dynamodb::Error {
            tracing::error!("DynamoDB query error: {:?}", e);
            e.into()
        },
    )?;

    let found_items: Vec<Item> = match query_output.items {
        Some(items) => items.iter().map(|v| v.into()).collect(),
        None => vec![],
    };

    let body = serde_json::to_string(&found_items).map_err(|e| -> serde_json::Error {
        tracing::error!("Serialization error: {:?}", e);
        e.into()
    })?;

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::Text(body))
        .map_err(|e| -> lambda_http::Error {
            tracing::error!("Response building error: {}", e);
            e.into()
        })?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let config = aws_config::from_env().region("us-east-1").load().await;
    let client: Client = Client::new(&config);

    run(service_fn(|_| get_items(&client))).await?;

    Ok(())
}
