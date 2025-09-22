
use axum::{
    extract::Form,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use reqwest::Client;
use serde_json::Value;



// --- Form payload from Africa's Talking ---
#[derive(Debug, Deserialize)]
struct UssdRequest {
    sessionId: String,
    serviceCode: String,
    phoneNumber: String,
    networkCode: String,
    text: String,
}




// --- Handler ---
async fn ussd_handler(Form(payload): Form<UssdRequest>) -> Response {
    // Build JSON payload for SpaceTimeDB reducer
    let json_payload = serde_json::json!({
        "sessionId": payload.sessionId,
        "phoneNumber": payload.phoneNumber,
        "networkCode": payload.networkCode,
        "serviceCode": payload.serviceCode,
        "text": payload.text,
    });

    let mut reply_text: String = "END Service unavailable".to_string();

        // Call SpaceTimeDB reducer over HTTP (assuming reducer is exposed at /call/handle_ussd)
    let client = Client::new();
    let spacetime_url = "http://127.0.0.1:3000/v1/database/gateway/call/handle_ussd"; // adjust for your SpacetimeDB instance

    let spacetime_sql_url = "http://127.0.0.1:3000/v1/database/gateway/sql";

    let response = client.post(spacetime_url).json(&json_payload).send().await;

    let sql_query = format!(
        "SELECT s.text \
        FROM ussd_session AS sess \
        JOIN ussd_screen AS s \
        ON sess.current_screen = s.name \
        WHERE sess.session_id = '{}';",
        payload.sessionId
    );


    let rpl = client.post(spacetime_sql_url)
    .bearer_auth("eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJoZXhfaWRlbnRpdHkiOiJjMjAwMWRkZDAwN2FhMzk2OGY5OTNjYTgzNGQzNzE1YzkzMjE5ZjkyMWI5NmI0OWMxMzQ5OTQxMjVjOTQ5OTBlIiwic3ViIjoiOTFjYjQ4MmEtNjc0ZC00M2I2LTk2YjgtYjUwZGUzMWI3MzRhIiwiaXNzIjoibG9jYWxob3N0IiwiYXVkIjpbInNwYWNldGltZWRiIl0sImlhdCI6MTc1ODIzNTkxOCwiZXhwIjpudWxsfQ.DtxquimduUhLhrnjc4LBpIMXR_Xbw7rQ4m1xTiQrKLGnKM-f9D6sR7ACYzmWX8hQ_mRjJbK9awGxBlS2gb4haQ") // load from creds
    .header("Content-Type", "text/plain") // 👈 tell it this is raw SQL
    .body(sql_query) // 👈 just the SQL text
    .send()
    .await;


    // let rply = rpl.unwrap();
    let body = rpl.unwrap().text().await.unwrap();

    println!("Response {:?}. ", body);

    if let Ok(json) = serde_json::from_str::<Value>(&body) {
        if let Some(row_value) = json
        .get(0)               // outer array
        .and_then(|v| v.get("rows"))
        .and_then(|rows| rows.get(0)) // first row
        .and_then(|row| row.get(0))   // first column
        .and_then(|val| val.as_str())
    {
        reply_text = row_value.to_string();
    }
    }

    // Respond in text/plain (required by Africa’s Talking)
    ([(axum::http::header::CONTENT_TYPE, "text/plain")], reply_text).into_response()
}

// --- Main server ---
#[tokio::main]
async fn main() {
    let app = Router::new().route("/ussd", post(ussd_handler));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("USSD bridge running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
