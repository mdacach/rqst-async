use miniserve::{Content, Request, Response};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

#[derive(serde::Deserialize)]
struct ChatRequest {
    messages: Vec<String>,
}

#[derive(serde::Serialize)]
struct ChatResponse {
    messages: Vec<String>,
}

fn chat(req: Request) -> Response {
    let Request::Post(text) = req else {
        return Err(miniserve::http::status::StatusCode::from_u16(404)
            .expect("hardcoded status code should be valid"));
    };

    let request_payload: ChatRequest =
        serde_json::from_str(&text).expect("string in chat request should be valid");

    let response = {
        let mut messages = request_payload.messages;
        messages.push(String::from("And how does that make you feel?"));
        ChatResponse { messages }
    };

    Ok(Content::Json(serde_json::json!(response).to_string()))
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run();
}
