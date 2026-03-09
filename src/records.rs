use crate::{SmritiClient, SmritiError};
use crate::model::*;
use serde_json::Value;

impl SmritiClient {

    pub async fn create_db_collection(
        &self,
        collection_name: &str,
    ) -> Result<CreateDbCollectionResponse, SmritiError> {

        let url = format!("{}/db/create-collection", self.base_url);

        let body = CreateDbCollectionRequest {
            collection_name: collection_name.to_string(),
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(SmritiError::RequestError)?;

        let status = response.status();
        let body_text = response.text().await.map_err(SmritiError::RequestError)?;

        if !status.is_success() {
            return Err(SmritiError::ServerError(body_text));
        }

        let result: CreateDbCollectionResponse =
            serde_json::from_str(&body_text).map_err(SmritiError::SerializationError)?;

        Ok(result)
    }


    pub async fn create_record(
        &self,
        collection_name: &str,
        payload: Value,
    ) -> Result<CreateRecordResponse, SmritiError> {

        let url = format!("{}/db/create-record", self.base_url);

        let body = CreateRecordRequest {
            collection_name: collection_name.to_string(),
            payload,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(SmritiError::RequestError)?;

        let status = response.status();
        let body_text = response.text().await.map_err(SmritiError::RequestError)?;

        if !status.is_success() {
            return Err(SmritiError::ServerError(body_text));
        }

        let result: CreateRecordResponse =
            serde_json::from_str(&body_text).map_err(SmritiError::SerializationError)?;

        Ok(result)
    }


    pub async fn get_record(
        &self,
        collection_name: &str,
        record_id: &str,
    ) -> Result<GetRecordResponse, SmritiError> {

        let url = format!("{}/db/get-record", self.base_url);

        let body = GetRecordRequest {
            collection_name: collection_name.to_string(),
            record_id: record_id.to_string(),
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(SmritiError::RequestError)?;

        let status = response.status();
        let body_text = response.text().await.map_err(SmritiError::RequestError)?;

        if !status.is_success() {
            return Err(SmritiError::ServerError(body_text));
        }

        let result: GetRecordResponse =
            serde_json::from_str(&body_text).map_err(SmritiError::SerializationError)?;

        Ok(result)
    }


    pub async fn llm_response_feedback_record(
        &self,
        payload: Value,
    ) -> Result<LlmResponseFeedbackResponse, SmritiError> {

        let url = format!("{}/db/llm-response-feedback-record", self.base_url);

        let body = LlmResponseFeedbackRequest {
            payload,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(SmritiError::RequestError)?;

        let status = response.status();
        let body_text = response.text().await.map_err(SmritiError::RequestError)?;

        if !status.is_success() {
            return Err(SmritiError::ServerError(body_text));
        }

        let result: LlmResponseFeedbackResponse =
            serde_json::from_str(&body_text).map_err(SmritiError::SerializationError)?;

use crate::model::{
    CreateCollectionResponse, CreateRecordRequest, GetRecordRequest, HybridQueryVectorRecordRequest, PayloadResponse, QueryRecordsRequest, QueryRecordsResponse, UpdateRecordRequest, VectorFreeSearch,VectorFreeSearchResponse
};
use crate::{SmritiClient, SmritiError};
use crate::model::*;
use serde_json::Value;
impl SmritiClient {
    pub async fn create_record(
    &self,
    req: CreateRecordRequest,
    ) -> Result<CreateCollectionResponse, SmritiError> {
    let response = self.http_client
        .post(format!("{}/db/create-record", self.base_url))
        .json(&req)
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        return Err(SmritiError::ServerError(body));
    }
    let result: CreateCollectionResponse = serde_json::from_str(&body)?;
    Ok(result)
}
    pub async fn query_records(
        &self,
        req: QueryRecordsRequest,
    ) -> Result<QueryRecordsResponse, SmritiError> {
        let url = format!("{}/db/query-records", self.base_url);
        let response = self.http_client
            .post(url)
            .json(&req)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            let error_message: serde_json::Value = serde_json::from_str(&body)?;
            return Err(SmritiError::ServerError(
                error_message["message"]
                    .as_str()
                    .unwrap_or("Unknown server error")
                    .to_string(),
                ));
        }
        let result: QueryRecordsResponse = serde_json::from_str(&body)?;
        Ok(result)
    }
    pub async fn delete_record(
    &self,
    req:GetRecordRequest
   )->Result<PayloadResponse, SmritiError>{
     let url = format!("{}/db/delete-record", self.base_url);
        let response = self.http_client
            .delete(url)
            .json(&req)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
               let error_message: serde_json::Value = serde_json::from_str(&body)?;
               return Err(SmritiError::ServerError(
                    error_message["message"]
                        .as_str()
                        .unwrap_or("Unknown server error")
                        .to_string(),
                    ));
        }
        let result: PayloadResponse = serde_json::from_str(&body)?;
        Ok(result)
   }
    pub async fn update_record(
    &self,
    req:UpdateRecordRequest
   )->Result<PayloadResponse, SmritiError>{
     let url = format!("{}/db/update-record", self.base_url);
        let response = self.http_client
            .put(url)
            .json(&req)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            let error_message: serde_json::Value = serde_json::from_str(&body)?;
            return Err(SmritiError::ServerError(
                error_message["message"]
                    .as_str()
                    .unwrap_or("Unknown server error")
                    .to_string(),
                ));
        }
        let result: PayloadResponse = serde_json::from_str(&body)?;
        Ok(result)
   }
    pub async fn query_vector_record_hybrid(
    &self,
    req:HybridQueryVectorRecordRequest
    )->Result<HybridQueryVectorRecordResponse, SmritiError>{
     let url = format!("{}/db/query-vector-hybrid", self.base_url);
        let response = self.http_client
            .post(url)
            .json(&req)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
         if !status.is_success() {
            let error_message: serde_json::Value = serde_json::from_str(&body)?;
            return Err(SmritiError::ServerError(
                error_message["message"]
                    .as_str()
                    .unwrap_or("Unknown server error")
                    .to_string(),
            ));
        }
        let result = serde_json::from_str(&body)?;
        Ok(result)
    }
    pub async fn query_vector_free_search(
    &self,
    req:VectorFreeSearch
    )->Result<VectorFreeSearchResponse,SmritiError>{
     let url = format!("{}/db/query-vector-free-search", self.base_url);
        let response = self.http_client
            .post(url)
            .json(&req)
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;
         if !status.is_success() {
            let error_message: serde_json::Value = serde_json::from_str(&body)?;
            return Err(SmritiError::ServerError(
                error_message["message"]
                    .as_str()
                    .unwrap_or("Unknown server error")
                    .to_string(),
                ));
        }
        let result = serde_json::from_str(&body)?;
        Ok(result)
    }
}