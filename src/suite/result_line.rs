use utility_parsers::{ok_or_failed, digits};

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
    pub suite_result<SuiteResult>,
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

#[cfg(test)]
mod tests {
    use nom::IResult;
    use std::fmt::Debug;

    use super::{SuiteResult, suite_result};

    fn assert_done<R: PartialEq + Debug>(l: IResult<&[u8], R>, r: R) {
        assert_eq!(
            l,
            IResult::Done(&b""[..], r)
        )
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
}
