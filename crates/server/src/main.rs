use miniserve::{Content, Request, Response};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(req: Request) -> Response {
    let Request::Post(_) = req else {
        return Err(miniserve::http::status::StatusCode::from_u16(404)
            .expect("hardcoded status code should be valid"));
    };
    let response = String::from("sample response text");
    Ok(Content::Json(response))
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run();
}
