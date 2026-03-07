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
                return Err(SmritiError::ServerError(body));
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
                return Err(SmritiError::ServerError(body));
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
            return Err(SmritiError::ServerError(body));
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
            return Err(SmritiError::ServerError(body));
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
            return Err(SmritiError::ServerError(body));
        }
        let result = serde_json::from_str(&body)?;
        Ok(result)
    }
}