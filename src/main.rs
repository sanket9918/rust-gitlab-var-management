use std::error::Error;

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
    /// Delete an env var -> Provide in this format <KEY>
    DeleteVar { key: Option<String> },
    ///Update the key -> Provide in this format <KEY> <VALUE>
    UpdateVar {
        key: Option<String>,
        value: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Initilize the project deps from the .env file
    let project_id = String::from(std::env::var("PROJECT_ID").expect("PROJECT_ID must be set."));
    let api_token = String::from(std::env::var("API_TOKEN").expect("API_TOKEN must be set."));

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
        SubOpArgs::DeleteVar { key } => {
            println!("Lets delete the provided key: {:?}", key);

            env_manage::requester::delete_var(&project_id, &api_token, key.as_deref().unwrap())
                .await?;

            println!("Key {:?} deleted successfully", key)
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
    }

    Ok(())
}
