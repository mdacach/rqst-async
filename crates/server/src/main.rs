use miniserve::{Content, Request, Response};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(_req: Request) -> Response {
    let response = String::from("sample response text");
    Ok(Content::Json(response))
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run();
}
