mod header;
mod utility_parsers;

#[macro_use]
extern crate nom;

use std::str;

use nom::{line_ending, digit, space};
use header::cargo_header;
use utility_parsers::rest_of_line;

named!(
    suite_line<&str>,
    do_parse!(
        ws!(
            alt!(tag!("Running") | tag!("Doc-tests"))
        ) >>
        name: rest_of_line >>
        (name)
    )
);

named!(
    suite_count<()>,
    do_parse!(
        ws!(tag!("running")) >>
        rest_of_line >>
        ()
    )
);

named!(
    ok<&str>,
    map!(tag!("ok"),
    |_| "pass")
);

named!(
    failed<&str>,
    map!(tag!("FAILED"),
    |_| "fail")
);

named!(
    ok_or_failed<&str>,
    alt!(ok | failed)
);

#[derive(Debug, PartialEq)]
pub struct Test<'a, 'b, 'c> {
    pub name: &'a str,
    pub status: &'b str,
    pub error: Option<&'c str>,
}

named!(
    test_result<Test>,
    do_parse!(
        tag!("test") >>
        space >>
        name: map_res!(
            take_until_s!(" ..."),
            str::from_utf8
        ) >>
        tag!(" ...") >>
        status: ws!(ok_or_failed) >>
        (Test {
            name: name,
            status: status,
            error: None
        })
    )
);

named!(
    test_results<Vec<Test> >,
    many0!(
        test_result
    )
);

named!(
    digits<i64>,
    map_res!(
        map_res!(
            ws!(digit),
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);

#[derive(Debug, PartialEq)]
pub struct SuiteResult<'a> {
    pub state: &'a str,
    pub passed: i64,
    pub failed: i64,
    pub ignored: i64,
    pub total: i64,
    pub measured: i64,
}

named!(
    suite_result<SuiteResult>,
    do_parse!(
        ws!(tag!("test result: ")) >>
        state: ok_or_failed >>
        char!('.') >>
        passed: digits >>
        tag!("passed;") >>
        failed: digits >>
        tag!("failed;") >>
        ignored: digits >>
        tag!("ignored;") >>
        measured: digits >>
        tag!("measured;") >>
        digits >>
        ws!(tag!("filtered out")) >>
        (SuiteResult {
          state:state,
          passed:passed,
          failed:failed,
          ignored:ignored,
          total: passed + failed + ignored,
          measured:measured
        })
    )
);

named!(
    fail_line<&str>,
    do_parse!(
        ws!(tag!("----")) >>
        name: map_res!(
            take_until!(" "),
            str::from_utf8
        ) >>
        ws!(tag!("stdout")) >>
        ws!(tag!("----")) >>
        (name)
    )
);

#[derive(Debug, PartialEq)]
pub struct Failure<'a, 'b> {
    pub name: &'a str,
    pub error: &'b str,
}

named!(
    failure<Failure>,
    do_parse!(
        name: fail_line >>
        error: rest_of_line >>
        opt!(
            tag!("note: Run with `RUST_BACKTRACE=1` for a backtrace.")
        ) >>
        line_ending >>
        line_ending >>
        (Failure {
            name:name,
            error:error
        })
    )
);

named!(failures<Vec<Failure> >, many1!(failure));

named!(fail_opt<Option<Vec<Failure> > >,
    opt!(
        do_parse!(
            ws!(
                tag!("failures:")
            ) >>
            f: failures >>
            take_until!(
                "test result: "
            ) >>
            (f)
        )
    )
);

#[derive(Debug, PartialEq)]
pub struct Suite<'a, 'b, 'c, 'd, 'e> {
    pub name: &'a str,
    pub state: &'b str,
    pub passed: i64,
    pub failed: i64,
    pub ignored: i64,
    pub measured: i64,
    pub total: i64,
    pub tests: Vec<Test<'c, 'd, 'e>>,
}

fn find_message_by_name<'a, 'b>(name: &str, failures: &Vec<Failure<'a, 'b>>) -> Option<&'b str> {
    failures.iter().find(|x| x.name == name).map(|x| x.error)
}

fn handle_parsed_suite<'a, 'b, 'c, 'd, 'e>(
    name: &'a str,
    tests: Vec<Test<'c, 'd, 'e>>,
    failures: Option<Vec<Failure<'e, 'e>>>,
    result: SuiteResult<'b>,
) -> Suite<'a, 'b, 'c, 'd, 'e> {
    let tests_with_failures = match failures {
        Some(xs) => {
            tests
                .iter()
                .map(|t| {
                    Test {
                        error: find_message_by_name(t.name, &xs),
                        name: t.name,
                        status: t.status,
                    }
                })
                .collect()
        }
        None => tests,
    };

    Suite {
        name: name,
        tests: tests_with_failures,
        state: result.state,
        total: result.total,
        passed: result.passed,
        failed: result.failed,
        ignored: result.ignored,
        measured: result.measured,
    }
}

named!(
    suite_parser<Suite>,
    do_parse!(
        name: suite_line >>
        suite_count >>
        tests: test_results >>
        failures: fail_opt >>
        result: suite_result >>
        (handle_parsed_suite(name, tests, failures, result))
    )
);

named!(
    suites_parser<Vec<Suite > >,
    many1!(suite_parser)
);

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
        name: "unknown",
        state: "fail",
        total: 1,
        passed: 0,
        failed: 1,
        ignored: 0,
        measured: 0,
        tests: vec![
          Test {
            name: "compile failed",
            status: "fail",
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

#[cfg(test)]
mod tests {
    use nom::IResult;
    use std::fmt::Debug;

    use super::{suite_line, suite_count, ok_or_failed, Test, test_result,
                test_results, digits, suite_result, SuiteResult, fail_line,
                failure, Failure, failures};

    fn assert_done<R: PartialEq + Debug>(l: IResult<&[u8], R>, r: R) {
        assert_eq!(
            l,
            IResult::Done(&b""[..], r)
        )
    }

    #[test]
    fn it_should_parse_suite_line() {
        let result = suite_line(
            &b"Running target/debug/deps/docker_command-be014e20fbd07382
"[..],
        );

        assert_done(result, "target/debug/deps/docker_command-be014e20fbd07382");
    }

    #[test]
    fn it_should_parse_suite_count() {
        let result = suite_count(
            &b"running 0 tests
"[..],
        );

        assert_done(result, ());
    }

    #[test]
    fn it_should_match_ok() {
        assert_done(ok_or_failed(&b"ok"[..]), "pass");
    }

    #[test]
    fn it_should_match_failed() {
        assert_done(ok_or_failed(&b"FAILED"[..]), "fail");
    }

    #[test]
    fn it_should_parse_test_result() {
        let result = test_result(&b"test it_runs_a_command ... ok"[..]);

        assert_done(
            result,
            Test {
                name: "it_runs_a_command",
                status: "pass",
                error: None,
            },
        );
    }

    #[test]
    fn it_should_parse_test_results() {
        let result = test_results(
            &b"test tests::it_should_parse_first_line ... ok
test tests::it_should_parse_a_status_line ... ok
test tests::it_should_parse_test_output ... ok
test tests::it_should_parse_suite_line ... FAILED
"
                [..],
        );

        assert_done(
            result,

            vec![
                Test {
                    name: "tests::it_should_parse_first_line",
                    status: "pass",
                    error: None
                },
                Test {
                    name: "tests::it_should_parse_a_status_line",
                    status: "pass",
                    error: None
                },
                Test {
                    name: "tests::it_should_parse_test_output",
                    status: "pass",
                    error: None
                },
                Test {
                    name: "tests::it_should_parse_suite_line",
                    status: "fail",
                    error: None
                }
            ],
        );
    }

    #[test]
    fn it_should_capture_digits() {
        assert_done(digits(b"10"), 10);
    }

    #[test]
    fn it_should_parse_a_suite_result() {
        let result = suite_result(
            &b"test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out"[..],
        );

        assert_done(
            result,
            SuiteResult {
                state: "fail",
                passed: 3,
                failed: 1,
                ignored: 0,
                total: 4,
                measured: 0,
            },
        );
    }

    #[test]
    fn test_fail_line() {
        let output = b"---- fail stdout ----";

        assert_done(fail_line(output), "fail");
    }

    #[test]
    fn test_failure() {
        let output = b"---- fail stdout ----
  thread 'fail' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `2`)', tests/integration_test.rs:16
note: Run with `RUST_BACKTRACE=1` for a backtrace.

";
        assert_done(
            failure(output),
            Failure {
                name: "fail",
                error: "thread 'fail' panicked at 'assertion failed: `(left == right)` \
                        (left: `1`, right: `2`)', tests/integration_test.rs:16",
            },
        );
    }

    #[test]
    fn test_failures() {
        let output = b"---- fail stdout ----
          thread 'fail' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `2`)', tests/integration_test.rs:16
note: Run with `RUST_BACKTRACE=1` for a backtrace.

        ---- fail2 stdout ----
          thread 'fail2' panicked at 'assertion failed: `(left == right)` (left: `3`, right: `2`)', tests/integration_test.rs:22


";

        assert_done(
            failures(output),
            vec![
                Failure {
                    name: "fail",
                    error: "thread 'fail' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `2`)', tests/integration_test.rs:16"
                },
                Failure {
                    name: "fail2",
                    error: "thread 'fail2' panicked at 'assertion failed: `(left == right)` (left: `3`, right: `2`)', tests/integration_test.rs:22"
                }
            ],
        );
    }

}
