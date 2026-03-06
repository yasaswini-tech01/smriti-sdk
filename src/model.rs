use serde::{Deserialize, Serialize};
use serde_json::Value;

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
