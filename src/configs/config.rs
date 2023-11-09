use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint{
    pub host: String,
    pub paths: Vec<String>,
    pub methods: Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipelines {
    pub api_endpoints: Vec<String>,
    pub policies: Vec<HashMap<String, HashMap<String, String>>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_endpoints: HashMap<String, Vec<Endpoint>>,
    pub service_endpoints: HashMap<String, Service>,
    pub pipelines: HashMap<String, Pipelines>
}
