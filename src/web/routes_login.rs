use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn routes() ->Router {
    Router::new().route("/api/v1/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");
    
    payload.validate()?;
    // FIXME: Use own logic
    

    // TODO: Set Cookies

    // TODO: Create the success body

    let body = Json(json!({
        "result": {
            "success" : true,
            "status" : 200
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String
}

impl LoginPayload {
    fn validate(&self) -> Result<()> {
        if self.username.is_empty() || self.password.is_empty() {
            return Err(Error::EmptyField(String::from("Username or password"))); // Adjust error type as needed
        }
        Ok(())
    }
}