use std::{error::Error, fs::read_to_string};

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use env_manage::requester::EnvVar;

mod env_manage;

#[derive(Parser)]
#[command(version,about,long_about= None)]
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
enum SubOpArgs {
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
fn read_lines(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Initilize the project deps from the .env file
    let project_id = String::from(std::env::var("PROJECT_ID").expect("PROJECT_ID must be set."));
    let api_token = String::from(std::env::var("API_TOKEN").expect("API_TOKEN must be set."));

    // Read the file

    // Get the command line args

    let args = Args::parse();

    // Build the cli matching instructions with the required argumments
    match args.op_name {
        SubOpArgs::GetVars => {
            println!("You have chosed to see the vars. Make sure that the ENV file contains the correct access_token and project_id");
            let get_res = env_manage::requester::get_all_vars(&project_id, &api_token).await?;

            println!("{}", serde_json::to_string_pretty(&get_res).unwrap())
        }
        SubOpArgs::CreateVar { key, value } => {
            println!("Lets start to add the variable in the Gitlab env");

            let env_var = &EnvVar {
                key_name: key.unwrap(),
                key_value: value.unwrap(),
            };
            let create_res =
                env_manage::requester::create_var(&project_id, &api_token, &env_var).await?;

            println!("{}", serde_json::to_string_pretty(&create_res).unwrap())
        }
        SubOpArgs::CreateMultipleVars { filename } => {
            println!("Lets start to add the variable in the Gitlab env");
            let lines = read_lines(filename.as_deref().unwrap());

            for ls in lines {
                let parts: Vec<&str> = ls.split("=").collect();

                let env_var = &EnvVar {
                    key_name: parts[0].to_string(),
                    key_value: parts[1].to_string(),
                };

                env_manage::requester::create_var(&project_id, &api_token, &env_var).await?;
            }
            println!("Env var addition complete");
        }
        SubOpArgs::DeleteVar { key } => {
            println!("Lets delete the provided key: {:?}", key);

            env_manage::requester::delete_var(&project_id, &api_token, key.as_deref().unwrap())
                .await?;

            println!("Key {:?} deleted successfully", key)
        }
        SubOpArgs::DeleteMultipleVars { filename } => {
            println!("Lets start deletion of the variable in the Gitlab env");
            let lines = read_lines(filename.as_deref().unwrap());

            for ls in lines {
                let parts: Vec<&str> = ls.split("=").collect();

                env_manage::requester::delete_var(&project_id, &api_token, &parts[0]).await?;
            }
            println!("Env var deletion complete");
        }
        SubOpArgs::UpdateVar { key, value } => {
            println!("Lets update the key {:?}", key);

            let env_var = &EnvVar {
                key_name: key.unwrap(),
                key_value: value.unwrap(),
            };

            let update_res =
                env_manage::requester::update_var(&project_id, &api_token, env_var).await?;

            println!("{}", serde_json::to_string_pretty(&update_res).unwrap());
        }
        SubOpArgs::UpdateMultipleVar { filename } => {
            println!("Lets start to update the variable in the Gitlab env");
            let lines = read_lines(filename.as_deref().unwrap());

            for ls in lines {
                let parts: Vec<&str> = ls.split("=").collect();

                let env_var = &EnvVar {
                    key_name: parts[0].to_string(),
                    key_value: parts[1].to_string(),
                };

                env_manage::requester::update_var(&project_id, &api_token, &env_var).await?;
            }
            println!("Env var updation complete");
        }
    }

    Ok(())
}
