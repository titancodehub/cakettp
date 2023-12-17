mod pkg;
mod command;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use command::{execute_request, run_program};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Exec {
        #[arg(short, long)]
        show_header: bool,
        #[arg(short, long)]
        file: String,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Exec { show_header, file }) => {
            match execute_request::execute(show_header, file.to_string()).await {
                Ok(()) =>{},
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        } 
        
        None => {
            match run_program::execute().await {
                Ok(()) => {}
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

}
