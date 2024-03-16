#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::serde::json;
use rocket::serde::json::{json, Json};
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorMessage {
    error: String,
}

impl ErrorMessage {
    pub fn new(error_message: &str) -> Self {
        Self {
            error: error_message.to_string(),
        }
    }
}

struct ApiKey {
    key: String,
}

impl ApiKey {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }

    pub fn getKey(&self) -> String {
        self.key.clone()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ErrorMessage;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Make sure there is an API key in the X-Api-Key header, or return an error
        // message stating that it's necessary
        if !req.headers().contains("X-Api-Key") || !req.headers().get("X-Api-Key").next().is_some()
        {
            Outcome::Error((
                Status::Unauthorized,
                ErrorMessage::new("An API key is required to access this method"),
            ))
        } else {
            let api_key_string = req.headers().get("X-Api-Key").next().unwrap();

            let api_key: ApiKey = ApiKey::new(api_key_string);
            Outcome::Success(api_key)
        }
    }
}

// #[catch(401)]
// fn unauthorized(result: Result<String, ErrorMessage>) -> Json<ErrorMessage> {
//     let errorMessage = result.err().unwrap();
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/sensitive")]
fn sensitive(guard: Result<ApiKey, ErrorMessage>) -> (Status, json::Value) {
    match guard {
        Result::Ok(key) => (Status::Ok, json!("{ \"response\": \"Cool, cool cool\" }")),
        Result::Err(e) => (Status::Unauthorized, json!(e)),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .register("/", catchers![unauthorized])
        .mount("/", routes![index, sensitive])
}
