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
    let all = params.len() / max;
    let mut count: usize = 0;
    let shared_diffs = Arc::new(Mutex::new(diffs));
    let shared_green_lines = Arc::new(Mutex::new(green_lines));

    let futures_data = futures::stream::iter(params.chunks(max).map(|chunk| {
        count += 1;
        let mut futures_data = FuturesData{
            remaining_params: Vec::new(),
            found_params: Vec::new()
        };

        let found_params: &Vec<String> = &found_params;
        let cloned_diffs = Arc::clone(&shared_diffs);
        let cloned_green_lines = Arc::clone(&shared_green_lines);

        async move {
            let query = &make_hashmap(&chunk, config.value_size);
            let response = request(config, client, query, reflections_count).await;

            if config.verbose > 0 && !config.disable_progress_bar {
                write!(
                    io::stdout(),
                    "{} {}/{}   \r"
                    &"-> ".bright_yellow(),
                    count,
                    all
                ).ok();
                
                io::stdout().flush().ok();
            }
        }
    }

    

}
