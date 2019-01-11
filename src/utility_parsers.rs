use nom::{digit, line_ending, not_line_ending};
use std::str;

named!(
    pub rest_of_line<&str>,
    do_parse!(
        content: map_res!(
            not_line_ending,
            str::from_utf8
        ) >>
        line_ending >>
        (content)
    )
);

named!(
    pub ok<&str>,
    map!(tag!("ok"),
    |_| "pass")
);

named!(
    pub failed<&str>,
    map!(tag!("FAILED"),
    |_| "fail")
);

named!(
    pub ok_or_failed<&str>,
    alt!(ok | failed)
);

named!(
    pub digits<i64>,
    map_res!(
        map_res!(
            ws!(digit),
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);

#[cfg(test)]
mod tests {
    use nom::IResult;
    use std::fmt::Debug;

    use super::{ok_or_failed, digits, rest_of_line};
    
    fn assert_done<R: PartialEq + Debug>(l: IResult<&[u8], R>, r: R) {
        assert_eq!(
            l,
            IResult::Done(&b""[..], r)
        )
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
    fn it_should_capture_digits() {
        assert_done(digits(b"10"), 10);
    }

    #[test]
    fn it_should_match_to_end_of_line() {
        assert_done(rest_of_line(&b"this is a test
"[..]), "this is a test");
    }
}
