use smriti_sdk::SmritiClient;
use smriti_sdk::model::VectorRecord;
use serde_json::json;

#[tokio::main]
async fn main() {


    let client = SmritiClient::new();
    let collection_name = "sdk_vector_test_example";
    println!("STEP 1: Creating collection...");
    let create_result = client
        .create_vector_collection(collection_name, 3)
        .await;

    println!("Create Result: {:?}\n", create_result);

    println!("STEP 2: Inserting single vector...");
    let insert_result = client
        .insert_vector(
            collection_name,
            vec![0.1, 0.2, 0.3],
            json!({
                "text": "single vector",
                "chunk_id": 1
            }),
        )
        .await;

    println!("Insert Vector Result: {:?}\n", insert_result);
   
    println!("STEP 3: Preparing vectors for bulk insert...");
    let vectors = vec![
        VectorRecord {
            vector: vec![0.4, 0.5, 0.6],
            payload: json!({
                "text": "bulk vector one",
                "chunk_id": 2
            }),
        },
        VectorRecord {
            vector: vec![0.7, 0.8, 0.9],
            payload: json!({
                "text": "bulk vector two",
                "chunk_id": 3
            }),
        },
    ];
println!("STEP 4: Bulk inserting vectors...");
let bulk_result = client
        .bulk_insert_vector(collection_name, vectors)
        .await;
println!("Bulk Insert Result: {:?}\n", bulk_result);

    println!("STEP 5: Querying vector...");
    let query_result = client
        .query_vector_record(
            collection_name,
            vec![0.1, 0.2, 0.3],
            5,
        )
        .await;

    println!("Query Result: {:?}\n", query_result);
}