use std::str;
use nom::space;

use utility_parsers::{ok_or_failed, rest_of_line};

mod result_line;
mod failure;

use self::result_line::{SuiteResult, suite_result};
use self::failure::{fail_opt, Failure};

#[derive(Debug, PartialEq)]
pub struct Test<'a, 'b, 'c> {
    pub name: &'a str,
    pub status: &'b str,
    pub error: Option<&'c str>,
}

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
    pub suites_parser<Vec<Suite > >,
    many1!(suite_parser)
);

#[cfg(test)]
mod tests {
    use nom::IResult;
    use std::fmt::Debug;

    use super::{suite_line, suite_count, Test, test_result, test_results};

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
}
