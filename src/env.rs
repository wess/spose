use std::env;

pub struct Env {
  pub project_id:String,
  pub api_endpoint:String,
  pub api_key:String,
}

impl Env {
  pub fn default() -> Self {
    Self {
      project_id: env::var("PROJECT_ID").unwrap(),
      api_endpoint: env::var("API_ENDPOINT").unwrap(),
      api_key: env::var("API_KEY").unwrap(),
    }
  }
}
