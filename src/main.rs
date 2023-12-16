use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, Subcommand};
mod pkg;
use pkg::display::console::ConsoleDisplay;
use pkg::http_parser::HttpParser;
use pkg::client::adapted_client_builder::AdaptedClientBuilder;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    Exec {
        #[arg(short, long)]
        show_header: bool,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let http_parser = HttpParser::new();
    let display= ConsoleDisplay::new();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Exec { show_header }) => {
            let mut should_show_header = show_header;
            if *show_header {
                should_show_header = show_header;
            }

            let http_file_request = http_parser.parse().unwrap();
            let builder = AdaptedClientBuilder::new();
            let mut client = builder.build(http_file_request).unwrap();
            let start_time = Instant::now();
            let response = client.send().await.unwrap();
            let finish_time = Instant::now();
            display.show(response, *should_show_header, finish_time.duration_since(start_time), client.get_method().as_str().to_owned()).await;
        } 

        Some(Commands::Test { list }) => {
            if *list {
                println!("Test Command Work")
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}
