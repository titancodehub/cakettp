use minus::Pager;
use reqwest::{Response, header::HeaderMap, StatusCode};
use console::{Style, Color};
use std::fmt::Write;
use colored_json::to_colored_json_auto;
pub struct ConsoleDisplay {}

impl ConsoleDisplay{
    pub fn new() -> ConsoleDisplay {
        ConsoleDisplay{}
    }

    pub async fn show(&self, res: Response, show_header: bool, duration: std::time::Duration, method: String){
      let mut output = Pager::new();
      let headers: reqwest::header::HeaderMap = res.headers().clone();
      let status: reqwest::StatusCode = res.status();
      let url = res.url().clone().as_str().to_owned();
      let body = res.text().await.unwrap().clone();
      self.show_info(&mut output, status, url, duration, method);
      if show_header {
        self.show_header(&mut output, headers);
      }
      self.show_body(&mut output, body);
      let _page_all = minus::page_all(output);
    }

    fn show_info(&self, output: &mut Pager, status: StatusCode, url: String, duration: std::time::Duration, method: String) {
      let st = Style::new();
      let title_st = st.clone().blue().bg(Color::Yellow).bold();
      let key_st = st.clone().bold().red();
      writeln!(output, "\n {} ", title_st.apply_to("Response")).unwrap();
      writeln!(output, "\n{}: {}", key_st.apply_to("Method"), method).unwrap();
      writeln!(output, "{}: {}", key_st.apply_to("Status"), status).unwrap();
      writeln!(output, "{}: {}", key_st.apply_to("Url"), url).unwrap();
      writeln!(output, "{}: {}ms", key_st.apply_to("Duration"), duration.as_millis()).unwrap();
    }

    fn show_header(&self,output: &mut Pager, headers: HeaderMap) {
      let st = Style::new();
      writeln!(output, "\n").unwrap();
      headers.iter().for_each(|(key, value)| {
        let format_key =  st.clone().bold().green().apply_to(key);
        let format_value = st.clone().bold().dim().apply_to(value);
        writeln!(output, "{}: {:?}", format_key, format_value).unwrap();
      });
    }

    fn show_body(&self, output:&mut Pager, body: String){
      let st = Style::new();
      writeln!(output, "\n{}", st.clone().bold().red().apply_to("Body")).unwrap();
      let body_json = serde_json::from_str::<serde_json::Value>(&body);
      if body_json.is_ok() {
        let body_json = body_json.unwrap();
        let colored = to_colored_json_auto(&body_json).unwrap();
        writeln!(output, "\n{}", colored).unwrap();
        return;
      }
      writeln!(output, "\n\n Cannot Parse Body").unwrap();
    }
}