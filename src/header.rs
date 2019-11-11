use utility_parsers::rest_of_line;

named!(
    compiling<()>,
    do_parse!(
      ws!(tag!("Compiling")) >>
      rest_of_line >>
      ()
    )
);

named!(
    downloading<()>,
    do_parse!(
      ws!(tag!("Downloading")) >>
      rest_of_line >>
      ()
    )
);

named!(
    downloaded<()>,
    do_parse!(
      ws!(tag!("Downloaded")) >>
      rest_of_line >>
      ()
    )
);

named!(
  installing<()>,
    do_parse!(
      ws!(tag!("Installing")) >>
      rest_of_line >>
      ()
    )
);

named!(
    updating<()>,
      do_parse!(
        ws!(tag!("Updating")) >>
        rest_of_line >>
        ()
      )
);

named!(
    finished<()>,
    do_parse!(
        ws!(tag!("Finished")) >>
        rest_of_line >>
        ()
    )
);

named!(
    pub cargo_header<()>,
    do_parse!(
        many0!(
            alt!(updating | downloading | downloaded | installing | compiling | finished)
        ) >>
        ()
    )
);

#[cfg(test)]
mod tests {
    use nom::IResult;
    use std::fmt::Debug;
    
    use super::{updating, downloading, downloaded, compiling, installing, finished, cargo_header};

    fn assert_done<R: PartialEq + Debug>(l: IResult<&[u8], R>, r: R) {
        assert_eq!(
            l,
            IResult::Done(&b""[..], r)
        )
    }

    #[test]
    fn it_should_parse_an_updating_line() {
        let output = &b"    Updating registry `https://github.com/rust-lang/crates.io-index`
"[..];

        assert_done(updating(output), ());
    }

        #[test]
    fn it_should_parse_a_downloaded_line() {
        let output = &b" Downloaded nvpair-sys v0.1.0
"[..];

        assert_done(downloaded(output), ())
    }
    
    #[test]
    fn it_should_parse_a_downloading_line() {
        let output = &b" Downloading nvpair-sys v0.1.0
"[..];

        assert_done(downloading(output), ())
    }

    #[test]
    fn it_should_parse_an_installing_line() {
        let output = &b" Installing cargo-test-junit v0.6.2
"[..];

        assert_done(installing(output), ())
    }

    #[test]
    fn it_should_parse_a_compiling_line() {
        let output = &b"   Compiling docker-command v0.1.0 (file:///Users/joegrund/projects/docker-command-rs)
"[..];

        assert_done(compiling(output), ());
    }

    #[test]
    fn it_should_parse_a_finished_line() {
        let output = &b"    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
"[..];
        
        assert_done(finished(output), ());
    }

    #[test]
    fn it_should_parse_a_full_header() {
        let output = &b"    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloaded nvpair-sys v0.1.0
 Downloading bindgen v0.30.0
 Downloading pkg-config v0.3.9
 Downloading clap v2.27.1
 Downloading which v1.0.3
 Downloading cfg-if v0.1.2
 Downloading lazy_static v0.2.10
 Downloading clang-sys v0.19.0
 Downloading log v0.3.8
 Downloading env_logger v0.4.3
 Downloading regex v0.2.2
 Downloading syntex_syntax v0.58.1
 Downloading aster v0.41.0
 Downloading quasi v0.32.0
 Downloading cexpr v0.2.2
 Downloading peeking_take_while v0.1.2
 Downloading textwrap v0.9.0
 Downloading unicode-width v0.1.4
 Downloading vec_map v0.8.0
 Downloading strsim v0.6.0
 Downloading atty v0.2.3
 Downloading bitflags v0.9.1
 Downloading ansi_term v0.9.0
 Downloading libc v0.2.33
 Downloading libloading v0.4.2
 Downloading glob v0.2.11
 Downloading aho-corasick v0.6.3
 Downloading utf8-ranges v1.0.0
 Downloading thread_local v0.3.4
 Downloading memchr v1.0.2
 Downloading regex-syntax v0.4.1
 Downloading unreachable v1.0.0
 Downloading void v1.0.2
 Downloading bitflags v0.8.2
 Downloading syntex_pos v0.58.1
 Downloading rustc-serialize v0.3.24
 Downloading unicode-xid v0.0.4
 Downloading syntex_errors v0.58.1
 Downloading term v0.4.6
 Downloading nom v3.2.1
 Downloading quasi_codegen v0.32.0
 Downloading syntex v0.58.1
   Compiling bitflags v0.9.1
   Compiling unicode-xid v0.0.4
   Compiling libc v0.2.33
   Compiling void v1.0.2
   Compiling ansi_term v0.9.0
   Compiling libloading v0.4.2
   Compiling utf8-ranges v1.0.0
   Compiling log v0.3.8
   Compiling lazy_static v0.2.10
   Compiling term v0.4.6
   Compiling unicode-width v0.1.4
   Compiling nvpair-sys v0.1.0
   Compiling pkg-config v0.3.9
   Compiling glob v0.2.11
   Compiling cfg-if v0.1.2
   Compiling regex-syntax v0.4.1
   Compiling peeking_take_while v0.1.2
   Compiling bitflags v0.8.2
   Compiling vec_map v0.8.0
   Compiling strsim v0.6.0
   Compiling rustc-serialize v0.3.24
   Compiling which v1.0.3
   Compiling atty v0.2.3
   Compiling memchr v1.0.2
   Compiling unreachable v1.0.0
   Compiling textwrap v0.9.0
   Compiling clang-sys v0.19.0
   Compiling syntex_pos v0.58.1
   Compiling nom v3.2.1
   Compiling aho-corasick v0.6.3
   Compiling thread_local v0.3.4
   Compiling clap v2.27.1
   Compiling syntex_errors v0.58.1
   Compiling cexpr v0.2.2
   Compiling regex v0.2.2
   Compiling syntex_syntax v0.58.1
   Compiling env_logger v0.4.3
   Compiling quasi v0.32.0
   Compiling syntex v0.58.1
   Compiling aster v0.41.0
   Compiling quasi_codegen v0.32.0
   Compiling bindgen v0.30.0
   Compiling libzfs-sys v0.1.0 (file:///vagrant/libzfs-sys)
    Finished dev [unoptimized + debuginfo] target(s) in 862.1 secs
"[..];

        assert_done(cargo_header(output), ());
    }
}
