use std::collections::HashMap;

use reqwest::{header::HeaderMap, Client};
use serde_json::Value;

#[derive(Debug)]
pub struct CreateEnvVar {
    pub key_name: String,
    pub key_value: String,
}

pub async fn get_all_vars(project_id: &str, access_token: &str) -> Result<Value, reqwest::Error> {
    let url = format!(
        "https://gitlab.com/api/v4/projects/{}/variables",
        project_id.to_string()
    );

    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert("PRIVATE-TOKEN", access_token.parse().unwrap());

    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(res)
}

pub async fn create_var(
    project_id: &str,
    access_token: &str,
    env_var: &CreateEnvVar,
) -> Result<Value, reqwest::Error> {
    let url = format!(
        "https://gitlab.com/api/v4/projects/{}/variables",
        project_id.to_string()
    );

    let client = Client::new();
    let mut headers = HeaderMap::new();

    headers.insert("PRIVATE-TOKEN", access_token.parse().unwrap());

    let mut params = HashMap::new();

    params.insert("key", &env_var.key_name);
    params.insert("value", &env_var.key_value);

    println!("{:?}", &params);

    let res = client
        .post(url)
        .form(&params)
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(res)
}
