use std::error::Error;

use clap::Parser;
use dotenv::dotenv;
use env_manage::requester::CreateEnvVar;

mod env_manage;
#[derive(Parser, Debug)]
#[command(version,about,long_about= None)]
struct Args {
    // Name
    #[arg(short, long)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args = Args::parse();

    println!("Hello {}", args.name);

    let env_var = CreateEnvVar {
        key_name: String::from(std::env::var("VAR_NAME").expect("VAR_NAME must be set.")),
        key_value: String::from(std::env::var("VAR_VALUE").expect("VAR_VALUE must be set.")),
    };

    let project_id = String::from(std::env::var("PROJECT_ID").expect("PROJECT_ID must be set."));
    let api_token = String::from(std::env::var("API_TOKEN").expect("API_TOKEN must be set."));

    let create_res = env_manage::requester::create_var(&project_id, &api_token, &env_var).await?;

    println!("Create Response: {:?}", create_res);
    // Get the env vars
    let get_res = env_manage::requester::get_all_vars(&project_id, &api_token).await?;

    println!("Get all vars response: {:?}", get_res);
    Ok(())
}
