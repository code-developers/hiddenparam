// use crates
use crate::requests::request;
use crate::structs::{Config, ResponseData};
use crate::diff::diff;

use lazy_static::lazy_static;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use rand::Rng;
use regex::Regex;
use reqwest::Client;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

lazy_static! {
    static ref FRAGMENT: AsciiSet = CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'<')
        .add(b'>')
        .add(b'`')
        .add(b'&')
        .add(b'#')
        .add(b';')
        .add(b'/')
        .add(b'=')
        .add(b'%');
    static ref RE_JSON_WORDS_WITHOUT_QUOTES: Regex =
        Regex::new(r#"^(\d+|null|false|true)$"#).unwrap();
    static ref RE_JSON_BRACKETS: Regex =
        Regex::new(r#"(?P<bracket>(\{"|"\}|\[("|\d)|("|\d)\]))"#).unwrap();
    static ref RE_JSON_COMMA_AFTER_DIGIT: Regex =
        Regex::new(r#"(?P<first>"[\w\.-]*"):(?P<second>\d+),"#).unwrap();
    static ref RE_JSON_COMMA_AFTER_BOOL: Regex =
        Regex::new(r#"(?P<first>"[\w\.-]*"):(?P<second>(false|null|true)),"#).unwrap();
}

//calls check_diffs & returns code and found diffs
pub fn compare(
    initial_response: &ResponseData,
    response: &ResponseData,
) -> (bool, Vec<String>) {

    let mut code: bool = true;
    let mut diffs: Vec<String> = Vec::new();

    if initial_response.code != response.code {
        code = false
    }

    //just push every found diff to the vector of diffs
    for diff in match diff(
        &initial_response.text,
        &response.text,
    ) {
        Ok(val) => val,
        Err(err) => {
            writeln!(io::stderr(), "Unable to compare: {}", err).ok();
            std::process::exit(1);
        }
    } {
        if !diffs.contains(&diff) {
            diffs.push(diff);
        } else {
            let mut c = 1;
            while diffs.contains(&[&diff, "(", &c.to_string(), ")"].concat()) {
                c += 1
            }
            diffs.push([&diff, " (", &c.to_string(), ")"].concat());
        }
    }

    (code, diffs)
}

//get possible parameters from the page code
pub fn heuristic(body: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();

    let re_special_chars = Regex::new(r#"[\W]"#).unwrap();

    let re_name = Regex::new(r#"(?i)name=("|')?"#).unwrap();
    let re_inputs = Regex::new(r#"(?i)name=("|')?[\w-]+"#).unwrap();
    for cap in re_inputs.captures_iter(body) {
        found.push(re_name.replace_all(&cap[0], "").to_string());
    }

    let re_var = Regex::new(r#"(?i)(var|let|const)\s+?"#).unwrap();
    let re_full_vars = Regex::new(r#"(?i)(var|let|const)\s+?[\w-]+"#).unwrap();
    for cap in re_full_vars.captures_iter(body) {
        found.push(re_var.replace_all(&cap[0], "").to_string());
    }

    let re_words_in_quotes = Regex::new(r#"("|')[a-zA-Z0-9]{3,20}('|")"#).unwrap();
    for cap in re_words_in_quotes.captures_iter(body) {
        found.push(re_special_chars.replace_all(&cap[0], "").to_string());
    }

    let re_words_within_objects = Regex::new(r#"[\{,]\s*[[:alpha:]]\w{2,25}:"#).unwrap();
    for cap in re_words_within_objects.captures_iter(body){
        found.push(re_special_chars.replace_all(&cap[0], "").to_string());
    }

    found.sort();
    found.dedup();
    found
}

pub fn generate_request(config: &Config, initial_query: &HashMap<String, String>) -> String {
    let mut query: HashMap<String, String> = HashMap::with_capacity(initial_query.len());
    for (k, v) in initial_query.iter() {
        query.insert(k.to_string(), v.replace("%random%_", ""));
    }

    let mut req: String = String::with_capacity(1024);
    req.push_str(&config.url);
    req.push('\n');
    req.push_str(&config.method);
    req.push(' ');

    if !config.as_body {
        let mut query_string = String::new();
        for (k, v) in query.iter() {
            query_string.push_str(k);
            query_string.push('=');
            query_string.push_str(v);
            query_string.push('&');
        }
        query_string.pop(); //remove the last &

        query_string = if config.encode {
            utf8_percent_encode(&query_string, &FRAGMENT).to_string()
        } else {
            query_string
        };

        req.push_str(&config.path.replace("%s", &query_string));
    }

    if config.http2 {
        req.push_str(" HTTP/2\n");
    } else {
        req.push_str(" HTTP/1.1\n");
    }

    if !config.headers.keys().any(|i| i.contains("Host")) {
        req.push_str(&("Host: ".to_owned() + &config.host));
        req.push('\n');
    }

    for (key, value) in config.headers.iter() {
        req.push_str(key);
        req.push_str(": ");
        req.push_str(&value.replace("{{random}}", &random_line(config.value_size)));
        req.push('\n');
    }

    let body: String = if config.as_body && !query.is_empty() {
        make_body(&config, &query)
    } else {
        config.body.to_owned()
    };

    if !body.is_empty() {
        req.push('\n');
        req.push_str(&body);
        req.push('\n');
    }

    req
}
