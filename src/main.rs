use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Serialize)]
struct Payload {
    time: u128,
}

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {
    let payload = Payload {
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros(),
    };

    let mut resp = Response::new();

    resp.set_body_json(&payload)?;

    Ok(resp)
}
