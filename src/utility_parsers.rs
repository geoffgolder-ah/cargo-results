use nom::{line_ending, not_line_ending};
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
