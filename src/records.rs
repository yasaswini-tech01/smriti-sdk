use crate::{SmritiClient, SmritiError};
use crate::model::*;
impl SmritiClient {
    pub async fn create_db_collection(
        &self,
        collection_name: &str,
    ) -> Result<CreateDbCollectionResponse, SmritiError> {

        let url = format!("{}/db/create-collection", self.base_url);

        let body = CreateDbCollectionRequest {
            collection_name: collection_name.to_string(),
        };

        let response = self.http_client
            .post(url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SmritiError::ServerError(
                response.text().await.unwrap_or_default(),
            ));
        }

        let result = response
            .json::<CreateDbCollectionResponse>()
            .await?;

        Ok(result)
    }
    pub async fn create_record(
    &self,
    collection_name: &str,
    payload: serde_json::Value,
) -> Result<CreateRecordResponse, SmritiError> {

    let url = format!("{}/db/create-record", self.base_url);

    let body = CreateRecordRequest {
        collection_name: collection_name.to_string(),
        payload,
    };

    let response = self.http_client
        .post(url)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(SmritiError::ServerError(
            response.text().await.unwrap_or_default(),
        ));
    }

    let result = response.json::<CreateRecordResponse>().await?;

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

    let response = self.http_client
        .post(url)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(SmritiError::ServerError(
            response.text().await.unwrap_or_default(),
        ));
    }

    let result = response.json::<GetRecordResponse>().await?;

    Ok(result)
}
  pub async fn llm_response_feedback_record(
    &self,
    payload: serde_json::Value,
) -> Result<LlmResponseFeedbackResponse, SmritiError> {

    let url = format!("{}/db/llm-response-feedback-record", self.base_url);

    let body = LlmResponseFeedbackRequest { payload };

    let response = self.http_client
        .post(url)
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(SmritiError::ServerError(
            response.text().await.unwrap_or_default(),
        ));
    }

    let result = response
        .json::<LlmResponseFeedbackResponse>()
        .await?;

    Ok(result)
}
}