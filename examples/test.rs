
use smriti_sdk::{SmrithiClient};
use smriti_sdk::model::{CreateCollectionResponse, CreateRecordRequest, DocumentQuery, GetRecordRequest, PayloadResponse, QueryRecordsRequest, QueryRecordsResponse, UpdateRecordRequest};
use serde_json::json;
#[tokio::main]
async fn main() {
    let client = SmrithiClient::new("http://localhost:6222".to_string());
//     let req = CreateRecordRequest {
//         collection_name: "chat".to_string(),
//         payload: json!({
//             "name": "Alice",
//             "age": 25
//         }),
//     };
//    let message: Result<CreateCollectionResponse, reqwest::Error> = client.create_record(req).await;
//    match message {
//         Ok(response) => {
//             println!("Success: {:?}", response);
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//         }
//     }
    let req = QueryRecordsRequest {
        document_queries: vec![
            DocumentQuery {
                collection_name: "chat".to_string(),
                query: json!({}), // Using the json! macro for an empty filter
                options: None,
            }
        ]
    };
    let message: anyhow::Result<QueryRecordsResponse> = client.query_records(req).await;
    match message {
        Ok(response) => {
            println!("Success: {:?}", response);
        }
        Err(err) => {
            eprintln!("Operation failed: {:?}", err);
        }
    }
    // let req=GetRecordRequest{
    //     collection_name:"chat".to_string(),
    //     record_id:"69a94cd5f7c61bc75c530c2d".to_string()
    // };
    // let message: anyhow::Result<PayloadResponse> = client.delete_record(req).await;
    // match message {
    //     Ok(response) => {
    //         println!("Success: {:?}", response);
    //     }
    //     Err(err) => {
    //         println!("Error: {:?}", err);
    //     }
    // }
    // let req=UpdateRecordRequest{
    //     collection_name:"chat".to_string(),
    //     record_id:"69a9574e114e78e3c84da06b".to_string(),
    //     document:{},
    // };
    // let message: anyhow::Result<PayloadResponse> = client.update_record(req).await;
    // match message {
    //     Ok(response) => {
    //         println!("Success: {:?}", response);
    //     }
    //     Err(err) => {
    //         println!("Error: {:?}", err);
    //     }
    // }












}