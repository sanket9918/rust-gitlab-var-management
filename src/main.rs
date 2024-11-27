use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use env_manage::{
    entrypoint::match_args,
    requester::{create_var, delete_var, get_all_vars, update_var},
};
use reqwest::Client;

mod env_manage;

#[derive(Parser)]
#[command(version="0.1.0",long_about= None,about="A rust based tool to manage Gitlab env variable for a project.")]
struct Args {
    /// The name of the intended operation to be done
    #[command(subcommand)]
    op_name: SubOpArgs,
}

// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// enum OpName {
//
//     GetVars,
//
//     CreateVar,
//     // DeleteVar,
//     // UpdateVar,
// }

#[derive(Subcommand)]
pub enum SubOpArgs {
    /// List down the variables
    GetVars,
    ///Create a new variable -> Provide in this format <KEY> <VALUE>
    CreateVar {
        key: Option<String>,
        value: Option<String>,
    },
    /// Create Multiple Vars based on the input in a file
    CreateMultipleVars { filename: Option<String> },
    /// Update multiple vars based on the input in a file
    UpdateMultipleVar { filename: Option<String> },
    /// Delete multiple vars based on the input in a file
    DeleteMultipleVars { filename: Option<String> },
    /// Delete an env var -> Provide in this format <KEY>
    DeleteVar { key: Option<String> },
    ///Update the key -> Provide in this format <KEY> <VALUE>
    UpdateVar {
        key: Option<String>,
        value: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Initilize the project deps from the .env file
    let project_id = String::from(std::env::var("PROJECT_ID").expect("PROJECT_ID must be set."));
    let api_token = String::from(std::env::var("API_TOKEN").expect("API_TOKEN must be set."));

    // Read the file

    // Get the command line args

    let args = Args::parse();

    // Get the client for the http request
    let client = Client::new();

    // Call the function that will match the args and do the suitable ops.
    match_args(&args.op_name, &project_id, &api_token, &client).await?;
    Ok(())
}
