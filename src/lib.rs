use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn hello_world(_: Request) -> Result<Response> {
    match std::fs::read_to_string("/test.txt") {
        Ok(txt) => Ok(http::Response::builder()
            .status(200)
            .body(Some(txt.into()))?),
        Err(e) => anyhow::bail!("Error, could not access test.txt: {}", e),
    }
}
