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
        .arg(Arg::with_name("proto")
            .long("proto")
            .help("Uses when the request file is present. (default is \"https\")")
            .takes_value(true)
            .requires("request")
            .conflicts_with("url")
        )
        .arg(
            Arg::with_name("wordlist")
                .short("w")
                .long("wordlist")
                .help("The file with parameters")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("parameter_template")
                .short("P")
                .long("param-template")
                .help("%k - key, %v - value. Example: --param-template 'user[%k]=%v&'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .help("Example: --body '{\"x\":{%s}}'\nAvailable variables: {{random}}")
                .value_name("body")
                .conflicts_with("request")
        )
        .arg(
            Arg::with_name("body-type")
                .short("t")
                .long("body-type")
                .help("Available: urlencode, json. (default is \"urlencode\")\nCan be detected automatically if --body is specified")
                .value_name("body type")
        )
        .arg(
            Arg::with_name("proxy")
                .short("x")
                .long("proxy")
                .value_name("proxy")
        )
        .arg(
            Arg::with_name("delay")
                .short("d")
                .long("delay")
                .value_name("Delay between requests in milliseconds")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output-format")
                .short("O")
                .long("output-format")
                .help("standart, json, url (default is \"standart\")")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("method")
                .short("X")
                .long("method")
                .value_name("method")
                .help("Available: GET, POST, PUT, PATCH, DELETE, HEAD. (default is \"GET\")")
                .takes_value(true)
                .conflicts_with("request")
        )
        .arg(
            Arg::with_name("headers")
                .short("H")
                .long("header")
                .help("Example: -H 'one:one' 'two:two'")
                .takes_value(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("as-body")
                .long("as-body")
                .help("Send parameters via body.\nBuilt in body types that can be detected automatically: json, urlencode")
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("Ignore 'binary data detected', 'the page is too huge', 'param_template lacks variables' error messages")
        )
        .arg(
            Arg::with_name("disable-response-correction")
                .long("disable-response-correction")
                .short("C")
                .help("Do not beautify responses before processing. Reduces accuracy.")
        )
        .arg(
            Arg::with_name("disable-custom-parameters")
                .long("disable-custom-parameters")
                .help("Do not check automatically parameters like admin=true")
        )
        .arg(
            Arg::with_name("disable-colors")
                .long("disable-colors")
        )
        .arg(
            Arg::with_name("disable-progress-bar")
                .long("disable-progress-bar")
        )
        .arg(
            Arg::with_name("replay-once")
                .long("replay-once")
                .help("If replay proxy is specified, send all found parameters within one request")
                .requires("replay-proxy")
        )
        .arg(
            Arg::with_name("replay-proxy")
                .takes_value(true)
                .long("replay-proxy")
                .help("Request target with every found parameter via replay proxy at the end")
        )
        .arg(
            Arg::with_name("custom-parameters")
                .long("custom-parameters")
                .help("Check these parameters with non-random values like true/false yes/no\n(default is \"admin bot captcha debug disable encryption env show sso test waf\")")
                .takes_value(true)
                .min_values(1)
                .conflicts_with("disable-custom-parameters")
        )
        .arg(
            Arg::with_name("custom-values")
                .long("custom-values")
                .help("Check custom parameters with these values (default is \"1 0 false off null true yes no\")")
                .takes_value(true)
                .min_values(1)
                .conflicts_with("disable-custom-parameters")
        )
        .arg(
            Arg::with_name("follow-redirects")
                .long("follow-redirects")
                .short("L")
                .help("Follow redirections")
        )
        .arg(
            Arg::with_name("encode")
                .long("encode")
                .help("Encodes query or body before a request, i.e & -> %26, = -> %3D\nList of chars to encode: \", `, , <, >, &, #, ;, /, =, %")
        )
        .arg(
            Arg::with_name("is-json")
                .long("is-json")
                .help("If the output is valid json and the content type does not contain 'json' keyword - specify this argument for a more accurate search")
        )
        .arg(
            Arg::with_name("test")
                .long("test")
                .help("Prints request and response")
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Verbose level 0/1/2 (default is 1)")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("save-responses")
                .long("save-responses")
                .help("Save matched responses to a directory")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("disable-cachebuster")
                .long("disable-cachebuster")
        )
        .arg(
            Arg::with_name("value_size")
                .long("value-size")
                .help("Custom value size. Affects {{random}} variables as well (default is 5)")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("learn_requests_count")
                .long("learn-requests")
                .help("Set the custom number of learning requests. (default is 9)")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("max")
                .short("m")
                .long("max")
                .help("Change the maximum number of parameters. (default is 128/192/256 for query and 512 for body)")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("concurrency")
                .short("c")
                .help("The number of concurrent requests (default is 1)")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("http2")
                .long("http2")
                .help("Prefer http/2 over http/1.1")
        );

    let args = app.clone().get_matches();
    
    let delay = match args.value_of("delay") {
        Some(val) => match val.parse() {
            Ok(val) => Duration::from_millis(val),
            Err(_) => {
                writeln!(io::stderr(), "Unable to parse 'delay' value").ok();
                std::process::exit(1);
            }
        },
        None::Duration::from_millis(0),
    };

    let max: usize = match args.value_of("max") {
        Some(val) => match val.parse() {
            Ok(val) => val,
            Err(_) => {
                writeln!(io::stderr(), "Unable to parse 'max' value ").ok();
                std::process::exit(1);
            }
        },
        None => {
            if args.is_present("as-body") {
                512
            } else {
                128
            }
        }
    };

}
