/*
 * Copyright 2017 Michael Krolikowski
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate clap;
extern crate regex;
extern crate time;

use clap::{AppSettings, Arg, ArgMatches};
use std::thread::sleep;
use time::{Duration, now_utc};

mod errors;

use errors::*;

mod logfile;

use logfile::*;

mod dateregex;

use dateregex::*;

fn relog(date_extraction_regex: &str, date_format: &str, filename: &str) -> Result<()> {
    let log_entry_parser = LogEntryParser::new(date_extraction_regex, date_format)?;
    let mut log_file = LogFile::new(filename, &log_entry_parser)?.peekable();

    let start_date_sys = now_utc();
    let mut start_date_log = {
        log_file.peek().and_then(|l| l.date)
    };
    for log_entry in log_file {
        let current_date_sys = now_utc();
        let current_date_log = log_entry.date;

        let sleep_duration = {
            match (start_date_log, current_date_log) {
                (Some(start_date_log), Some(current_date_log)) => std::cmp::max(Duration::zero(), (current_date_log - start_date_log) - (current_date_sys - start_date_sys)),
                _ => Duration::zero()
            }
        };

        if start_date_log.is_none() {
            start_date_log = current_date_log
        }

        if sleep_duration > Duration::zero() {
            if let Ok(sleep_duration) = sleep_duration.to_std() {
                sleep(sleep_duration)
            }
        }
        println!("{}", log_entry.line);
    }

    Ok(())
}

fn run() -> Result<()> {
    let matches: ArgMatches = app_from_crate!()
        .global_setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("date_extraction_regex")
            .short("r")
            .long("regex")
            .value_name("regex")
            .help("regex to extract date from log messages (default is generated from date format)"))
        .arg(Arg::with_name("date_format")
            .short("f")
            .long("format")
            .value_name("format")
            .default_value("%Y-%m-%d %H:%M:%S")
            .help("date format of logfile as specified in strftime(3)"))
        .arg(Arg::with_name("logfile")
            .help("logfile to read")
            .required(true))
        .get_matches();

    let date_format = matches.value_of("date_format").ok_or(Error::from("date_format missing"))?;
    let logfile = matches.value_of("logfile").ok_or(Error::from("logfile missing"))?;
    let date_extraction_regex = matches
        .value_of("date_extraction_regex")
        .ok_or(Error::from("date_extraction_regex missing"))
        .map(String::from)
        .or(
            regex_from_date_format(&date_format.as_bytes())
                .to_result()
                .chain_err(|| "failed to create regex from date_format")
        )?;

    relog(&date_extraction_regex, date_format, logfile)
}

quick_main!(run);
