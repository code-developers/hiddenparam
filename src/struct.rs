use std::{collections::HashMap, time::Duration};


##[derive(Debug)]
pub struct ResponseData {
    pub text: String,
    pub code: u16
    pub reflected_params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FuturesData {
    pub remaining_params: Vec<String>
    pub found_params: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Config {
    pub method: String,
    pub initial_url: String,
    pub url: String,
    pub host: String,
    pub path: String,
    pub wordlist: String,
    pub parameter_template: String,
    pub custom_parameters: HashMap<String, Vec<String>>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub body_type: String,
    pub proxy: String,
    pub output_file: String,
    pub output_format: String,
    pub save_responses: String,
    pub force: bool,
    pub disable_response_correction: bool,
    pub disable_custom_parameters: bool,
    pub disable_progress_bar: bool,
    pub replay_once: bool,
    pub replay_proxy: String,
    pub follow_redirects: bool,
    pub encode: bool,
    pub test: bool,
    pub as_body: bool,
    pub verbose: u8,
    pub is_json: bool,
    pub disable_cachebuster: bool,
    //pub verify: bool,
    pub delay: Duration,
    pub value_size: usize,
    pub learn_requests_count: usize,
    pub max: usize,
    pub concurrency: usize,
    pub http2: bool
}