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