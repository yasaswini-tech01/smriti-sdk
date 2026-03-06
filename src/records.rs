use crate::model::{
    CreateCollectionResponse, CreateRecordRequest, GetRecordRequest, HybridQueryVectorRecordRequest, PayloadResponse, QueryRecordsRequest, QueryRecordsResponse, UpdateRecordRequest, VectorFreeSearch
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
    println!("Status: {:?}", status);
    println!("Body: {:?}", body);
    if !status.is_success() {
        return Err(SmritiError::ServerError(body));
    }
    let result: CreateCollectionResponse = serde_json::from_str(&body)?;
    Ok(result)
    // if !response.status().is_success() {
    //     return Err(SmritiError::ServerError(response.text().await?));
    // }
    // let result = response.json::<CreateCollectionResponse>().await?;
    // Ok(result)
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
        println!("Status: {:?}", status);
        println!("Body: {:?}", body);
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
        println!("Status: {:?}", status);
        println!("Body: {:?}", body);
        if !status.is_success() {
                return Err(SmritiError::ServerError(body));
        }
        let result: PayloadResponse = serde_json::from_str(&body)?;
        Ok(result)
   }
   
//     pub async fn update_record(
//     &self,
//     req:UpdateRecordRequest
//    )->Result<PayloadResponse>{
//      let url = format!("{}/db/update-record", self.base_url);
//         let res = self.client
//             .put(url)
//             .json(&req)
//             .send()
//             .await?;
//         let status = res.status();
//         let text = res.text().await?;
//         println!("Status: {}", status);
//         println!("Raw Body: {}", text);
//         if text.trim().is_empty() {
//             return Err(anyhow::anyhow!("Server returned an empty response with status {}", status));
//         }
//         let body: PayloadResponse = serde_json::from_str(&text)
//             .with_context(|| format!("Failed to parse update response: {}", text))?;
//         Ok(body)
//    }

    // pub async fn query_vector_record_hybrid(
    // &self,
    // req:HybridQueryVectorRecordRequest
    // )->Result<PayloadResponse>{
    //  let url = format!("{}/db/query-vector-hybrid", self.base_url);
    //     let res = self.client
    //         .post(url)
    //         .json(&req)
    //         .send()
    //         .await?;
    //     let status = res.status();
    //     let text = res.text().await?;
    //     println!("Status: {}", status);
    //     println!("Raw Body: {}", text);
    //     if text.trim().is_empty() {
    //         return Err(anyhow::anyhow!("Server returned an empty response with status {}", status));
    //     }
    //     let body: PayloadResponse = serde_json::from_str(&text)
    //         .with_context(|| format!("Failed to parse delete response: {}", text))?;
    //     Ok(body)
    // }

    // pub async fn query_vector_free_search(
    // &self,
    // req:VectorFreeSearch
    // )->Result<PayloadResponse>{
    //  let url = format!("{}/db/query-vector-free-search", self.base_url);
    //     let res = self.client
    //         .post(url)
    //         .json(&req)
    //         .send()
    //         .await?;
    //     let status = res.status();
    //     let text = res.text().await?;
    //     println!("Status: {}", status);
    //     println!("Raw Body: {}", text);
    //     if text.trim().is_empty() {
    //         return Err(anyhow::anyhow!("Server returned an empty response with status {}", status));
    //     }
    //     let body: PayloadResponse = serde_json::from_str(&text)
    //         .with_context(|| format!("Failed to parse delete response: {}", text))?;
    //     Ok(body)
    // }
}