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

pub async fn random_request(
    config: &Config,
    client: &Client,
    reflections: usize,
    max: usize,
) -> ResponseData {
    request(
        &config,
        &client,
        &make_hashmap(
            &(0..max).map(|_| random_line(config.value_size)).collect::<Vec<String>>(),
            config.value_size
        ),
        reflections
    ).await
}

fn create_request(
    url: &str,
    body: String,
    config: &Config,
    client: &Client
) -> reqwest::RequestBuilder {
    let mut client = if config.as_body {
        match config.method.as_str() {
            "GET" => client.get(url).body(body),
            "POST" => client.post(url).body(body),
            "PUT" => client.put(url).body(body),
            "PATCH" => client.patch(url).body(body),
            "DELETE" => client.delete(url).body(body),
            "HEAD" => client.head(url).body(body),
            _ => {
                writeln!(io::stderr(), "Method is not supported").ok();
                std::process::exit(1);
            },
        }
    } else {
        match config.method.as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "PATCH" => client.patch(url),
            "DELETE" => client.delete(url),
            "HEAD" => client.head(url),
            _ => {
                writeln!(io::stderr(), "Method is not supported").ok();
                std::process::exit(1);
            }
        }
    };

    client = if config.as_body && !config.disable_cachebuster {
        client.query(&[(random_line(config.value_size), random_line(config.value_size))])
    } else {
        client
    };

    client = if !config.as_body && !config.body.is_empty() {
        client.body(config.body.clone())
    } else {
        client
    };

    for (key, value) in config.headers.iter() {
        client = client.header(key, value.replace("{{random}}", &random_line(config.value_size)));
    }

    client
}