use reqwest::Client;

use crate::env_manage::requester::EnvVar;
use crate::env_manage::util::read_lines;
use crate::SubOpArgs;
use crate::{create_var, delete_var, get_all_vars, update_var};

pub async fn match_args(
    args: &SubOpArgs,
    project_id: &str,
    api_token: &str,
    client: &Client,
) -> anyhow::Result<()> {
    match args {
        SubOpArgs::GetVars => {
            println!("You have chosed to see the vars. Make sure that the ENV file contains the correct access_token and project_id");
            let get_res = get_all_vars(&project_id, &api_token, &client).await?;

            println!("{}", serde_json::to_string_pretty(&get_res).unwrap())
        }
        SubOpArgs::CreateVar { key, value } => {
            println!("Lets start to add the variable in the Gitlab env");

            let env_var = EnvVar {
                key_name: key.as_deref().unwrap().to_string(),
                key_value: value.as_deref().unwrap().to_string(),
            };
            let create_res = create_var(&project_id, &api_token, &env_var, &client).await?;

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

                create_var(&project_id, &api_token, &env_var, &client).await?;
            }
            println!("Env var addition complete");
        }
        SubOpArgs::DeleteVar { key } => {
            println!("Lets delete the provided key: {:?}", key);

            delete_var(&project_id, &api_token, key.as_deref().unwrap(), &client).await?;

            println!("Key {:?} deleted successfully", key)
        }
        SubOpArgs::DeleteMultipleVars { filename } => {
            println!("Lets start deletion of the variable in the Gitlab env");
            let lines = read_lines(filename.as_deref().unwrap());

            for ls in lines {
                let parts: Vec<&str> = ls.split("=").collect();

                delete_var(&project_id, &api_token, &parts[0], &client).await?;
            }
            println!("Env var deletion complete");
        }
        SubOpArgs::UpdateVar { key, value } => {
            println!("Lets update the key {:?}", key);

            let env_var = &EnvVar {
                key_name: key.as_deref().unwrap().to_string(),
                key_value: value.as_deref().unwrap().to_string(),
            };

            let update_res = update_var(&project_id, &api_token, env_var, &client).await?;

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

                update_var(&project_id, &api_token, &env_var, &client).await?;
            }
            println!("Env var updation complete");
        }
    }
    Ok(())
}
