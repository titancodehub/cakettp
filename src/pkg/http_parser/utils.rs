use chrono;
use uuid;
use rand;
use regex::Regex;

use super::regex::COMMENT_REGEX;
const UUID: &str = "{{$uuid}}";
const RANDOM_INT: &str = "{{$randomInt}}";
const TIMESTAMP: &str = "{{$timestamp}}";
const DATETIME: &str = "{{$datetime}}";
const LOCAL_DATETIME: &str = "{{$localDatetime}}";

pub fn replace_placeholder(inp: String) -> String {
  let mut out = inp.clone();
  out = out.replace(UUID, uuid::Uuid::new_v4().to_string().as_str());
  out = out.replace(RANDOM_INT, rand::random::<u32>().to_string().as_str());
  out = out.replace(TIMESTAMP, chrono::Utc::now().timestamp().to_string().as_str());
  out = out.replace(DATETIME, chrono::Utc::now().to_string().as_str());
  out = out.replace(LOCAL_DATETIME, chrono::Local::now().to_string().as_str());
  return out
}

pub fn clean_comment(inp: String) -> String {
  let re = Regex::new(COMMENT_REGEX).unwrap();
  let clean = re.replace_all(&inp, "");
  return clean.to_string();
}
