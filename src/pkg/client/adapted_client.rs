use anyhow::Error;
use reqwest::{Client, Method, Response};
use serde_json::Value;

pub struct AdaptedClient {
  client: Client,
  url: String,
  body: Option<Value>,
  method: Method,
}

impl AdaptedClient {
  pub fn new() -> Self {
    return AdaptedClient {
      client: Client::new(),
      url: String::new(),
      body: None,
      method: Method::default(),
    };
  }

  pub fn set_url(&mut self, url: String){
   self.url = url;
  }

  pub fn set_body(&mut self, body: Option<Value>){
    self.body = body;
  }

  pub fn set_method(&mut self, method:Method){
    self.method = method;
  }

  pub fn get_method(&self) -> Method {
    return self.method.clone();
  }

  pub async fn send(&mut self) -> Result<Response, Error> {
    let mut request = self.client.request(self.method.clone(), self.url.clone());
    if !self.body.is_none(){
      request = request.body::<String>(self.body.clone().unwrap().to_string());
    }
    let response = self.client.execute(request.build().unwrap()).await?;
    return Ok(response);
  }
}
