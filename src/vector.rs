use crate::{SmritiClient, SmritiError};
use crate::model::*;
use serde_json::Value;

impl SmritiClient {

    pub async fn create_vector_collection(
        &self,
        collection_name: &str,
        vector_size: u64,
    ) -> Result<CreateCollectionResponse, SmritiError> {

        let url = format!("{}/vector/create-collection", self.base_url);
        let body = CreateVectorCollectionRequest {
            collection_name: collection_name.to_string(),
            vector_size,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(SmritiError::ServerError(
                response.text().await.unwrap_or_default(),
            ));
        }
        let result = response.json::<CreateCollectionResponse>().await?;
        Ok(result)
    }
    pub async fn insert_vector(
        &self,
        collection_name: &str,
        vector: Vec<f32>,
        payload: Value,
    ) -> Result<InsertVectorResponse, SmritiError> {

        let url = format!("{}/vector/insert-vector", self.base_url);

        let body = InsertVectorRequest {
            collection_name: collection_name.to_string(),
            vector,
            payload,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SmritiError::ServerError(
                response.text().await.unwrap_or_default(),
            ));
        }

        let result = response.json::<InsertVectorResponse>().await?;
        Ok(result)
    }
    pub async fn bulk_insert_vector(
        &self,
        collection_name: &str,
        vectors: Vec<VectorRecord>,
    ) -> Result<BulkInsertVectorResponse, SmritiError> {

        let url = format!("{}/vector/bulk-insert", self.base_url);

        let body = BulkInsertVectorRequest {
            collection_name: collection_name.to_string(),
            vectors,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SmritiError::ServerError(
                response.text().await.unwrap_or_default(),
            ));
        }

        let result = response.json::<BulkInsertVectorResponse>().await?;
        Ok(result)
    }
    pub async fn query_vector_record(
        &self,
        collection_name: &str,
        query_vector: Vec<f32>,
        top_k: u32,
    ) -> Result<Value, SmritiError> {

        let url = format!("{}/vector/query-record", self.base_url);

        let body = QueryVectorRequest {
            collection_name: collection_name.to_string(),
            query_vector,
            top_k,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SmritiError::ServerError(
                response.text().await.unwrap_or_default(),
            ));
        }

        // returning raw JSON to avoid schema mismatch
        let result = response.json::<Value>().await?;

        Ok(result)
    }
}