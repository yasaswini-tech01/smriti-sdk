use serde::{Deserialize, Serialize};
use serde_json::Value;


// Vector Collection
#[derive(Serialize)]
pub struct CreateVectorCollectionRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,

    #[serde(rename = "vectorSize")]
    pub vector_size: u64,
}

#[derive(Deserialize, Debug)]
pub struct CreateCollectionResponse {
    pub message: String,
}
/// Insert Vector
#[derive(Serialize)]
pub struct InsertVectorRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub vector: Vec<f32>,
    pub payload: Value,
}
#[derive(Deserialize, Debug)]
pub struct InsertVectorResponse {
    pub message: String,
}

/// Bulk Insert Vectors
#[derive(Serialize)]
pub struct BulkInsertVectorRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,

    pub vectors: Vec<VectorRecord>,
}
#[derive(Serialize)]
pub struct VectorRecord {
    pub vector: Vec<f32>,
    pub payload: Value,
}
#[derive(Deserialize, Debug)]
pub struct BulkInsertVectorResponse {
    pub message: String,
}

/// Query Vector
#[derive(Serialize)]
pub struct QueryVectorRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub query_vector: Vec<f32>,
    pub top_k: u32,
}
#[derive(Deserialize, Debug)]
pub struct QueryVectorResponse {
    pub results: Vec<serde_json::Value>,
}

/// Create DB Collection
#[derive(Serialize)]
pub struct CreateDbCollectionRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
}
#[derive(Deserialize, Debug)]
pub struct CreateDbCollectionResponse {
    pub message: String,
}

/// Create Record
#[derive(Serialize)]
pub struct CreateRecordRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub payload: Value,
}
#[derive(Deserialize, Debug)]
pub struct CreateRecordResponse {
    pub data: Value,
}

// Get Record
#[derive(Serialize)]
pub struct GetRecordRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    #[serde(rename = "recordId")]
    pub record_id: String,
}
#[derive(Deserialize, Debug)]
pub struct GetRecordResponse {
    pub data: Value,
}

// LLM Response Feedback
#[derive(Serialize)]
pub struct LlmResponseFeedbackRequest {
    pub payload: Value,
}
#[derive(Deserialize, Debug)]
pub struct LlmResponseFeedbackResponse {
    pub data: Value,
}