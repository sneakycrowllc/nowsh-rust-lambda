use http::{StatusCode, header};
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use std::error::Error;

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let uri = request.uri();
    let response = Respone::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "type/html")
        .body(format!(
            "You made a request to the following URL: {}",
            uri
        ))
        .expect("failed to render reponse");
    Ok(response)
        
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
