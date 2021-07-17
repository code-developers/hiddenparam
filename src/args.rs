use crate::{structs::Config, utils::{parse_request, adjust_body}};
use clap::{crate_version, App, AppSettings, Arg};
use std::{collections::HashMap, fs, time::Duration, io::{self, Write}};
use url::Url;

