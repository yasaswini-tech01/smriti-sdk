// use crate::model::{CreateRecordRequest,CreateCollectionResponse,QueryRecordsRequest, QueryRecordsResponse};
// use crate::SmrithiClient;
// use anyhow::Result;
// impl SmrithiClient{
//    pub async fn create_record(
//     &self,
//     req: CreateRecordRequest,
//     ) -> Result<CreateCollectionResponse, reqwest::Error> {
//         let url = format!("{}/db/create-record", self.base_url);
//         let res = self.client
//             .post(url)
//             .json(&req)
//             .send()
//             .await?;
//         let body = res.json::<CreateCollectionResponse>().await?;
//         println!("{:#?}", body);
//         Ok(body)
//     }

//     pub async fn query_records(
//     &self,
//     req: QueryRecordsRequest,
// ) -> Result<QueryRecordsResponse, reqwest::Error> {
//     let url = format!("{}/db/query_records", self.base_url);
//     let res = self.client
//         .post(url)
//         .json(&req)
//         .send()
//         .await?;

//     let text = res.text().await?;

//     println!("Raw response: {}", text);

//     let body: QueryRecordsResponse = match serde_json::from_str(&text) {
//     Ok(v) => v,
//     Err(e) => {
//         println!("JSON parse error: {}", e);
//         return Err("Error:{}",e)
//         }
//     };
//     println!("{:#?}", body);

//     Ok(body)
// }
// }
use crate::model::{
    CreateCollectionResponse, CreateRecordRequest, GetRecordRequest, HybridQueryVectorRecordRequest, PayloadResponse, QueryRecordsRequest, QueryRecordsResponse, UpdateRecordRequest, VectorFreeSearch
};

use crate::SmrithiClient;
use anyhow::{Result, Context}; // Context adds helpful error messages
impl SmrithiClient {

    pub async fn create_record(
        &self,
        req: CreateRecordRequest,
    ) -> Result<CreateCollectionResponse> {
        let url = format!("{}/db/create-record", self.base_url);
        let res = self.client
            .post(url)
            .json(&req)
            .send()
            .await?;
        let body = res.json::<CreateCollectionResponse>().await?;
        Ok(body)
    }
 
    pub async fn query_records(
        &self,
        req: QueryRecordsRequest,
    ) -> Result<QueryRecordsResponse> {
        let url = format!("{}/db/query-records", self.base_url);
        let res = self.client
            .post(url)
            .json(&req)
            .send()
            .await
            .context("Failed to send POST request to query_records")?;
        let text = res.text().await?;
        let body: QueryRecordsResponse = serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse JSON response: {}", text))?;
        Ok(body)
    }

   pub async fn delete_record(
    &self,
    req:GetRecordRequest
   )->Result<PayloadResponse>{
     let url = format!("{}/db/delete-record", self.base_url);
        let res = self.client
            .delete(url)
            .json(&req)
            .send()
            .await?;
        let status = res.status();
        let text = res.text().await?;
        println!("Status: {}", status);
        println!("Raw Body: {}", text);
        if text.trim().is_empty() {
            return Err(anyhow::anyhow!("Server returned an empty response with status {}", status));
        }
        let body: PayloadResponse = serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse delete response: {}", text))?;
        Ok(body)
   }
   
    pub async fn update_record(
    &self,
    req:UpdateRecordRequest
   )->Result<PayloadResponse>{
     let url = format!("{}/db/update-record", self.base_url);
        let res = self.client
            .put(url)
            .json(&req)
            .send()
            .await?;
        let status = res.status();
        let text = res.text().await?;
        println!("Status: {}", status);
        println!("Raw Body: {}", text);
        if text.trim().is_empty() {
            return Err(anyhow::anyhow!("Server returned an empty response with status {}", status));
        }
        let body: PayloadResponse = serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse update response: {}", text))?;
        Ok(body)
   }

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