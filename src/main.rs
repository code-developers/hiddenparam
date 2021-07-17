extern crate hiddenparam;
use colored::*;
use reqwest::Client;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    time::Duration,
};
use hiddenparam::{
    args::get_config,
    logic::cycles,
    requests::{empty_reqs, random_request, request},
    structs::Config,
    utils::{compare, generate_data, heuristic, make_hashmap, random_line, read_lines, create_output},
};

#[cfg(windows)]
#[tokio::main]
async fn main() {
    colored::control::set_virtual_terminal(true).unwrap();
    run().await;
}

#[cfg(not(windows))]
#[tokio::main]
async fn main() {
    run().await;
}x