use askama::Template;
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

mod meta;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    source_url: &'a str,
    version: &'a str,
    revision: &'a str,
    author_name: &'a str,
    author_url: &'a str,
}

#[http_component]
fn handle_spin_http_rust_example(req: Request) -> anyhow::Result<impl IntoResponse> {
    if req.path() != "/" {
        return Ok(Response::new(404, "Not Found."));
    }
    if req.method() != &Method::Get && req.method() != &Method::Head {
        return Ok(Response::new(405, "Method Not Allowed."));
    }
    let template = IndexTemplate {
        source_url: meta::SOURCE_URL,
        version: meta::VERSION,
        revision: meta::REVISION,
        author_name: meta::AUTHOR_NAME,
        author_url: meta::AUTHOR_URL,
    };
    let html = template.render().unwrap();
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(html)
        .build())
}
