use crate::{structs::Config, utils::{parse_request, adjust_body}};
use clap::{crate_version, App, AppSettings, Arg};
use std::{collections::HashMap, fs, time::Duration, io::{self, Write}};
use url::Url;

pub fn get_config() -> (Config, usize) {

    let app = App::new("hiddenparam")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author("krishpranav")
        .about("Hidden parameters discovery suite")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .help("You can add a custom injection point with %s")
            .takes_value(true)
            .conflicts_with("request")
        )
        .arg(Arg::with_name("request")
            .short("r")
            .long("request")
            .help("The file with raw http request")
            .takes_value(true)
            .conflicts_with("url")
        )

    }
}
