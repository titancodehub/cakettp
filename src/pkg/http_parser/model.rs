use reqwest::Method;
use serde_json::Value;

#[derive(Debug)]
pub struct HttpFileParsed {
  pub method: String,
  pub url: String,
  pub body: Option<Value>,
}

impl HttpFileParsed {
  pub fn get_method(&self) -> Method {
    match self.method.as_str() {
      "GET" => Method::GET,
      "POST" => Method::POST,
      "PUT" => Method::PUT,
      "DELETE" => Method::DELETE,
      _ => Method::GET,
    }
  }
}
