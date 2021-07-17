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
    pub wordlist: String
    pub body: String
    pub proxy: String
    pub replay_once: bool
}