
use smriti_sdk::{SmritiClient, SmritiError};
use smriti_sdk::model::{CreateCollectionResponse, CreateRecordRequest, DocumentQuery, GetRecordRequest, HybridQueryVectorRecordRequest, HybridQueryVectorRecordResponse, PayloadResponse, QueryRecordsRequest, QueryRecordsResponse, UpdateRecordRequest, VectorFreeSearch,VectorFreeSearchResponse};
use serde_json::json;
#[tokio::main]
async fn main() {
    let client = SmritiClient::new();
//     let req = CreateRecordRequest {
//         collection_name: "chat".to_string(),
//         payload: json!({
//             "name": "Alice",
//             "age": 25
//         }),
//     };
//    let message: Result<CreateCollectionResponse, SmritiError> = client.create_record(req).await;
//    match message {
//         Ok(response) => {
//             println!("Success: {:?}", response);
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//         }
//     }
//     let req = QueryRecordsRequest {
//         document_queries: vec![
//             DocumentQuery {
//                 collection_name: "chat".to_string(),
//                 query: json!({}), // Using the json! macro for an empty filter
//                 options: None,
//             }
//         ]
//     };
//     let message: Result<QueryRecordsResponse, SmritiError> = client.query_records(req).await;
//     match message {
//         Ok(response) => {
//             println!("Success: {:?}", response);
//         }
//         Err(err) => {
//             eprintln!("Operation failed: {:?}", err);
//         }
//     }
//     let req: GetRecordRequest=GetRecordRequest{
//         collection_name:"chat".to_string(),
//         record_id:"69aae62354915dd8e417be10".to_string()
//     };
//     let message: Result<PayloadResponse, SmritiError>= client.delete_record(req).await;
//     match message {
//         Ok(response) => {
//             println!("Success: {:?}", response);
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//         }
//     }
    // let req=UpdateRecordRequest{
    //     collection_name:"chat".to_string(),
    //     record_id:"69ab042c54915dd8e417be13".to_string(),
    //     document: json!({
    //     "name": "priya123",
    //     "age": 35
    //     }),
    // };
    // let message: Result<PayloadResponse, SmritiError> = client.update_record(req).await;
    // match message {
    //     Ok(response) => {
    //         println!("Success: {:?}", response);
    //     }
    //     Err(err) => {
    //         println!("Error: {:?}", err);
    //     }
    // }
    // let req = HybridQueryVectorRecordRequest {
    // collection_name: "new-test".to_string(),
    // query_vector: vec![0.21, 0.67, 0.45, 0.12, 0.89],
    // metadata_query: Some(json!({
    //     "term": {
    //         "age": 25
    //     }
    // })),
    // vector_weight: 0.7,
    // metadata_weight: 0.3,
    // top_k: 2,
    // };
    // let message: Result<HybridQueryVectorRecordResponse, SmritiError> = client.query_vector_record_hybrid(req).await;
    // match message {
    //     Ok(response) => {
    //         println!("Success: {:?}", response);
    //     }
    //     Err(err) => {
    //         println!("Error: {:?}", err);
    //  }
    // }
    let req = VectorFreeSearch{
        collection_name: "new-test".to_string(),
        query:json! ({
            "term": {
            "age": 25
            }
         })
    };
    let message: Result<VectorFreeSearchResponse, SmritiError> = client.query_vector_free_search(req).await;
    match message {
        Ok(response) => {
            println!("Success: {:?}", response);
        }
        Err(err) => {
            println!("Error: {:?}", err);
     }
    }












}