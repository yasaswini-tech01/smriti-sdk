use smriti_sdk::SmritiClient;
use serde_json::json;

#[tokio::main]
async fn main() {

    let client = SmritiClient::new("http://localhost:6222");

    let result = client
        .llm_response_feedback_record(
            json!({
                "responseId": "69aa7e7059569a73b489b03d",
                "rating": 5,
                "feedback": "Good answer"
            })
        )
        .await;

    println!("Feedback Result: {:?}", result);
}