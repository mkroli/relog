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

named!(date_format_field<&[u8], &str>, alt!(
    map!(char!('%'), |_| r"\%")
  | map!(char!('Y'), |_| r"\d{4}")
  | map!(char!('D'), |_| r"\d{2}/\d{2}/\d{4}")
  | map!(char!('F'), |_| r"\d{4}-\d{2}-\d{2}")
  | map!(char!('T'), |_| r"\d{2}:\d{2}:\d{2}")
  | map!(char!('e'), |_| r"( \d|\d{2})")
  | map!(one_of!("mCdHMS"), |_| r"\d{2}")
  | map!(take_str!(1), |_| r"\w+")
));

named!(format_field<&[u8], String>, do_parse!(
  char!('%') >>
  date_format_field: map!(date_format_field, String::from) >>
  (date_format_field)
));

named!(character<&[u8], String>,
  map!(take_str!(1), String::from)
);

named!(escaped_character<&[u8], String>,
  map!(one_of!(r"[](){}|?+-*^$\."), |c| format!(r"\{}", c))
);

named!(regexes_from_date_format<&[u8], Vec<String>>, many0!(complete!(alt!(format_field | escaped_character | character))));

named!(pub regex_from_date_format<&[u8], String>, map!(regexes_from_date_format, |l: Vec<String>| format!("({})", l.concat())));
