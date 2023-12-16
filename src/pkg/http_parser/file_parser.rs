use std::fs;
use anyhow::Error;
use regex::Regex;
use serde_json::Value;

use super::model::HttpFileParsed;

const METHOD_REGEX: &str = r"^(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT|TRACE|LOCK|UNLOCK|PROPFIND|PROPPATCH|COPY|MOVE|MKCOL|MKCALENDAR|ACL|SEARCH)\s+";

enum ParserState {
  RequestLine,
  HeaderLine,
  BodyLine,
}

pub struct FileParser {}
impl FileParser {
  pub fn new() -> Self {
    FileParser {}
  
  }

  pub fn parse(&self) -> Result<HttpFileParsed, Error> {
    // follow https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html
    let mut request_lines: Vec<String> =[].to_vec();
    let mut header_lines: Vec<String> = [].to_vec();
    let mut body_lines: String = "".to_string();

    // read raw text and split into lines
    let raw_content: String = fs::read_to_string("src/pkg/http_parser/example.http")
    .expect("Something went wrong reading the file");
    let mut content: Vec<String> = raw_content.lines().into_iter().map(|line| line.to_string()).collect();
    let mut state: ParserState = ParserState::RequestLine;
    let mut current_line: String;
    while !content.is_empty() {
      current_line = content[0].clone();
      content.remove(0);
      let mut next_line="".to_string();
      if content.len() > 0 {
        next_line = content[0].clone();
      } 
        match state {
          ParserState::RequestLine  => {
            request_lines.push(current_line);
            if next_line =="" {
              continue;  
            }

            let _ = next_line.trim();
            if next_line != "" {
              state = ParserState::HeaderLine;
              continue;
            } else {
              state = ParserState::BodyLine;
              let _ = content.remove(0);
              continue;
            }
          }

          ParserState::HeaderLine  => {
            let _ = next_line.trim();
            if next_line == "" {
              content.remove(0);
              state = ParserState::BodyLine;
              continue;
            }

            header_lines.push(current_line);
          }

          ParserState::BodyLine  => {
            body_lines.push_str(current_line.as_str());
            continue;
          }
        }
    }

    // Extract method and url
    let re = Regex::new(METHOD_REGEX)?;
    let upper_case_line = request_lines[0].to_ascii_uppercase();
    let find_method = re.find(upper_case_line.as_str());
    if find_method.is_none() {
      return Err(Error::msg("invalid method"));
    }
    let method = find_method.unwrap().as_str();
    let find_url = upper_case_line.replace(method, "");

    // Parse Body
    let mut binding: Value = Value::Null;
    if body_lines.len() != 0 {
      binding = serde_json::from_str(body_lines.as_str())?;
    }

    return Ok(HttpFileParsed {
      url: find_url.trim().to_ascii_lowercase().to_string(),
      method: method.trim().to_string(),
      body: if binding == Value::Null { None } else { Some(binding) },
    });
  }
}
