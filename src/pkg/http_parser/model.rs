use reqwest::Method;
use serde_json::Value;

#[derive(Debug)]

pub struct HttpRequestBlock {
  pub name: String,
  pub description: String,
  pub tag: String
}
pub struct HttpModel {
  pub method: String,
  pub url: String,
  pub body: Option<Value>,
}

impl HttpModel {
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
