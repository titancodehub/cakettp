
use std::fs::read_dir;
use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use anyhow::Error;
use super::execute_request;

const DEFAULT_REQUEST_DIRECTORY: &str = "./cakettp";

#[derive(PartialEq, Eq)]
enum PageName {
  RequestSelection
}

pub async fn execute() -> Result<(), Error> {
  let page = PageName::RequestSelection;
  if page == PageName::RequestSelection {
    let selected = select_request()?;
    execute_selected_request(selected).await?;
  }
  
  return Ok(());
}

async fn execute_selected_request(file_path: String) -> Result<(), Error> {
  execute_request::execute(&false, file_path).await?;
  return Ok(());
}

fn select_request() -> Result<String, Error> {
  let list_dir = read_dir(DEFAULT_REQUEST_DIRECTORY)?;
  let mut request_definitions: Vec<String> = Vec::new();
    for element in list_dir {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "http" {
              request_definitions.push(path.display().to_string());
            }
        }
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
    .with_prompt("Select a request")
    .items(&request_definitions)
    .interact()?;
    return Ok(request_definitions[selection].to_string());
}
