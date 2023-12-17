use anyhow::Error;
use std::time::Instant;

use crate::pkg::display::console::ConsoleDisplay;
use crate::pkg::http_parser::http_parser::HttpParser;
use crate::pkg::client::adapted_client_builder::AdaptedClientBuilder;

pub async fn execute(show_header: &bool, file_path: String) -> Result<(), Error> {
  let http_parser = HttpParser::new();
  let display= ConsoleDisplay::new();
  
  let mut should_show_header = show_header;
  if *show_header {
      should_show_header = show_header;
  }
  
  let http_file_request = http_parser.parse(file_path)?;
  let builder = AdaptedClientBuilder::new();
  let mut client = builder.build(http_file_request)?;
  let start_time = Instant::now();
  let response = client.send().await?;
  let finish_time = Instant::now();
  display.show(response, *should_show_header, finish_time.duration_since(start_time), client.get_method().as_str().to_owned()).await;
  return Ok(());
}
