use std::str;
use nom::line_ending;

#[derive(Debug, PartialEq)]
pub struct Failure<'a, 'b> {
    pub name: &'a str,
    pub error: &'b str,
}

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

named!(
    failure<Failure>,
    do_parse!(
        name: fail_line >>
        error: map_res!(take_until!("\n\n"), str::from_utf8) >>
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

named!(pub fail_opt<Option<Vec<Failure> > >,
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

#[cfg(test)]
mod tests {
    use nom::IResult;
    use std::fmt::Debug;

    use super::{fail_line, failure, Failure, failures, fail_opt};

    fn assert_left<R: PartialEq + Debug>(l: IResult<&[u8], R>, r: R, remaining: &[u8]) {
        assert_eq!(
            l,
            IResult::Done(remaining, r)
        )
    }
                                                    
    fn assert_done<R: PartialEq + Debug>(l: IResult<&[u8], R>, r: R) {
        assert_eq!(
            l,
            IResult::Done(&b""[..], r)
        )
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

    #[test]
    fn test_fail_opt() {
        let output = &b"failures:

---- fail stdout ----
thread 'fail' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `2`)', tests/integration_test.rs:16
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- fail2 stdout ----
thread 'fail2' panicked at 'assertion failed: `(left == right)` (left: `3`, right: `2`)', tests/integration_test.rs:22


failures:
        fail
        fail2

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
"[..];

        assert_left(fail_opt(output),
                    Some(vec![
                        Failure {
                            name: "fail",
                            error: "thread 'fail' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `2`)', tests/integration_test.rs:16",
                        },
                        Failure {
                            name: "fail2",
                            error: "thread 'fail2' panicked at 'assertion failed: `(left == right)` (left: `3`, right: `2`)', tests/integration_test.rs:22",
                        },
                    ]),
        &b"test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
"[..]);
    }
}
