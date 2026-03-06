use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Serialize)]
pub struct CreateRecordRequest {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub payload: Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCollectionResponse {
     #[serde(rename = "data")]
    pub msg:Value,
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

#[derive(Serialize,Deserialize,Debug)]
pub struct PayloadResponse {
    #[serde(rename = "data")]
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


#[derive(Serialize,Deserialize, Debug)]
pub struct VectorFreeSearch {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub query: serde_json::Value,
}
