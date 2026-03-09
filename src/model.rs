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
    #[serde(rename = "data")]
    pub message: Value,
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

#[derive(Debug, Serialize)]
pub struct CreateRecordRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub payload: Value,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRecordsRequest {
    #[serde(rename = "documentQueries")]
    pub document_queries: Vec<DocumentQuery>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRecordsResponse {
    pub data: Vec<Vec<Value>>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentQuery {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub query: serde_json::Value,
    pub options: Option<QueryOptions>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryOptions {
    populate: Option<Vec<PopulateOptions>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PopulateOptions {
    pub path: String,
    pub from: String
}

#[derive(Serialize,Deserialize, Debug)]
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

#[derive(Debug, Deserialize)]
pub struct PayloadResponse {
    #[serde(rename = "data")]
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message: String,
}


#[derive(Serialize,Deserialize, Debug)]
pub struct UpdateRecordRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    #[serde(rename = "recordId")]
    pub record_id: String,
    pub document: serde_json::Value,
}


#[derive(Serialize,Deserialize, Debug)]
pub struct HybridQueryVectorRecordRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    #[serde(rename = "queryVector")]
    pub query_vector: Vec<f32>,
    #[serde(rename = "metadataQuery")]
    pub metadata_query: Option<serde_json::Value>,
    #[serde(rename = "vectorWeight")]
    pub vector_weight: f32,
    #[serde(rename = "metadataWeight")]
    pub metadata_weight: f32,
    #[serde(rename = "topK")]
    pub top_k: usize,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct HybridQueryVectorRecordResponse{
    pub data: Vec<SearchResult>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub payload: Option<Value>,
}
#[derive(Serialize,Deserialize, Debug)]
pub struct VectorFreeSearch {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub query: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorFreeSearchResponse {
    pub data: Vec<SearchResult>,
}





