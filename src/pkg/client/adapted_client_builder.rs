use anyhow::Error;

use crate::pkg::http_parser::model::HttpFileParsed;

use crate::pkg::client::adapted_client::AdaptedClient;

pub struct AdaptedClientBuilder {
}

impl AdaptedClientBuilder {
    pub fn new() -> Self {
        AdaptedClientBuilder {
        }
    }

    pub fn build(&self, cfg: HttpFileParsed) -> Result<AdaptedClient , Error>{
      let url = cfg.url.clone();
      let method = cfg.get_method();
      let body = cfg.body; 
      let mut client = AdaptedClient::new();
      client.set_url(url);
      client.set_method(method);
      client.set_body(body);
      return Ok(client);
    }
}
