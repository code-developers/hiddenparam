use crate::{
    structs::{Config, ResponseData, Stable},
    utils::{compare, beautify_html, beautify_json, make_body, make_query, make_hashmap, random_line},
};
use colored::*;
use reqwest::Client;
use std::{
    error::Error,
    time::Duration,
    collections::{BTreeMap, HashMap},
    io::{self, Write},
};

//makes first requests and checks page behavior
pub async fn empty_reqs(
    config: &Config,
    initial_response: &ResponseData,
    reflections_count: usize,
    count: usize,
    client: &Client,
    max: usize,
) -> (Vec<String>, Stable) {
    let mut stable = Stable {
        body: true,
        reflections: true,
    };
    let mut diffs: Vec<String> = Vec::new();

    for i in 0..count {
        let response = random_request(config, client, reflections_count, max).await;

        //progress bar
        if config.verbose > 0 && !config.disable_progress_bar {
            write!(
                io::stdout(),
                "{} {}/{}       \r",
                &"-> ".bright_green(),
                i,
                count
            ).ok();
            io::stdout().flush().unwrap_or(());
        }

        if response.text.len() > 25 * 1024 * 1024 && !config.force {
            writeln!(io::stderr(), "[!] {} the page is too huge", &config.url).ok();
            std::process::exit(1)
        }

        if !response.reflected_params.is_empty() {
            stable.reflections = false;
        }

        let (is_code_the_same, new_diffs) = compare(initial_response, &response);

        if !is_code_the_same {
            writeln!(
                io::stderr(),
                "[!] {} the page is not stable (code)",
                &config.url
            ).ok();
            std::process::exit(1)
        }

        for diff in new_diffs {
            if !diffs.iter().any(|i| i == &diff) {
                diffs.push(diff);
            }
        }
    }

    let response = random_request(config, client, reflections_count, max).await;

    for diff in compare(initial_response, &response).1 {
        if !diffs.iter().any(|i| i == &diff) {
            if config.verbose > 0 {
                writeln!(
                    io::stdout(),
                    "{} the page is not stable (body)",
                    &config.url
                ).ok();
            }
            stable.body = false;
            return (diffs, stable);
        }
    }
    (diffs, stable)
}