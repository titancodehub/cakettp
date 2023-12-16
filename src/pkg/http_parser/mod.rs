pub mod file_parser;
pub mod model;
pub mod variable_parser;

use model::HttpModel;
use file_parser::FileParser;
use anyhow::Error;

#[derive(Debug)]
pub struct HttpParser {}
impl HttpParser {
  pub fn new() -> Self {
    HttpParser {}
  }
  pub fn parse(&self)-> Result<HttpModel, Error>{
    let file_parser = FileParser::new();
    let content = file_parser.parse()?;
    return Ok(content)
  }
}
