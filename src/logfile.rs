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

use errors::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use time::{Tm, strptime};

#[derive(Debug)]
pub struct LogEntry {
    pub date: Option<Tm>,
    pub line: String
}

pub struct LogEntryParser<'a> {
    date_extraction_regex: Regex,
    date_format: &'a str
}

impl<'a> LogEntryParser<'a> {
    pub fn new(date_extraction_regex: &str, date_format: &'a str) -> Result<LogEntryParser<'a>> {
        let regex = Regex::new(date_extraction_regex).chain_err(|| format!("failed to create regex {}", date_extraction_regex))?;
        Ok(LogEntryParser {
            date_extraction_regex: regex,
            date_format: date_format
        })
    }

    pub fn parse(&self, entry: String) -> LogEntry {
        let date_time = self
            .date_extraction_regex
            .captures(&entry)
            .and_then(|c| c.get(1))
            .and_then(|c| strptime(c.as_str(), self.date_format).ok());

        LogEntry {
            date: date_time,
            line: entry
        }
    }
}

pub struct LogFile<'a> {
    reader: Lines<BufReader<File>>,
    log_entry_parser: &'a LogEntryParser<'a>
}

impl<'a> LogFile<'a> {
    pub fn new(filename: &str, log_entry_parser: &'a LogEntryParser<'a>) -> Result<LogFile<'a>> {
        let file = File::open(filename).chain_err(|| "failed to open file")?;
        let reader = BufReader::new(file).lines();
        Ok(LogFile {
            reader: reader,
            log_entry_parser: log_entry_parser
        })
    }
}

impl<'a> Iterator for LogFile<'a> {
    type Item = LogEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next().and_then(|l| {
            l.ok().map(|l| self.log_entry_parser.parse(l))
        })
    }
}
