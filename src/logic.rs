use crate::{
    requests::{random_request, request},
    structs::{Config, ResponseData, Stable, FuturesData},
    utils::{compare, make_hashmap, random_line, generate_request},
};
use colored::*;
use futures::stream::StreamExt;
use std::sync::Arc;
use parking_lot::Mutex;
use reqwest::Client;

use std::{
    collections::HashMap,
    io::{self, Write},
};

pub async fn cycles(
    first: bool,
    config: &Config,
    initial_response: &ResponseData,
    diffs: &mut Vec<String>,
    params: &[String],
    stable: &Stable,
    reflections_count: usize,
    client: &Client,
    max: usize,
    green_lines: &mut HashMap<String, usize>,
    remaining_params: &mut Vec<Vec<String>>,
    found_params: &mut Vec<String>,
) {
    
}