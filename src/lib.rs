mod header;
mod utility_parsers;
mod suite;

#[macro_use]
extern crate nom;

use std::str;

use header::cargo_header;
pub use suite::{Suite, Test};
use suite::suites_parser;

named!(
  compile_error<Vec<Suite > >,
  do_parse!(
    ws!(tag!("error")) >>
    opt_res!(
      do_parse!(
        char!('[') >>
        take_until_and_consume!("]") >>
        ()
      )
    ) >>
    ws!(char!(':')) >>
    error: map_res!(
            take_till!(|c| c == 0x0),
            str::from_utf8
        ) >>
    (vec![Suite {
        name: "unknown".to_string(),
        state: "fail".to_string(),
        total: 1,
        passed: 0,
        failed: 1,
        ignored: 0,
        measured: 0,
        tests: vec![
          Test {
            name: "compile failed".to_string(),
            status: "fail".to_string(),
            error: Some(error.into())
          }
        ]
    }])
  )
);

named!(
    pub cargo_test_result_parser<Vec<Suite > >,
    do_parse!(
        cargo_header >>
        suites: alt!(suites_parser | compile_error) >>
        (suites)
    )
);
