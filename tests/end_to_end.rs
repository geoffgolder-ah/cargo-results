extern crate cargo_results;
extern crate nom;

use nom::IResult;
use std::fmt::Debug;
use cargo_results::{Test, cargo_test_result_parser, Suite};

fn assert_done<R: PartialEq + Debug>(l: IResult<&[u8], R>, r: R) {
    assert_eq!(
        l,
        IResult::Done(&b""[..], r)
    )
}

#[test]
fn it_should_parse_successful_test_output() {
    let output = &b"    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
       Running target/debug/cargo_test_junit-83252957c74e106d

running 2 tests
test tests::it_should_match_failed ... ok
test tests::it_should_parse_first_line ... ok


  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  "
        [..];

    let result = cargo_test_result_parser(output);

    assert_done(
        result,
        vec![Suite {
            name: "target/debug/cargo_test_junit-83252957c74e106d".to_string(),
            state: "pass".to_string(),
            tests: vec![
                Test {
                    name: "tests::it_should_match_failed".to_string(),
                    status: "pass".to_string(),
                    error: None
                },
                Test {
                    name: "tests::it_should_parse_first_line".to_string(),
                    status: "pass".to_string(),
                    error: None
                }
            ],
            passed: 2,
            failed: 0,
            ignored: 0,
            measured: 0,
            total: 2
        }],
    );
}

#[test]
fn test_fail_run() {
    let output = b"  Compiling blah v0.1.0 (file:blah)
        Finished debug [unoptimized + debuginfo] target(s) in 0.32 secs
        Running target/debug/deps/docker_command-be014e20fbd07382

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

        Running target/debug/integration_test-d4fc68dd5824cbb9

running 3 tests
test fail ... FAILED
test fail2 ... FAILED
test it_runs_a_command ... ok

failures:

---- fail stdout ----
thread 'fail' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `2`)', tests/integration_test.rs:16
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- fail2 stdout ----
thread 'fail2' panicked at 'assertion failed: `(left == right)` (left: `3`, right: `2`)', tests/integration_test.rs:22


failures:
        fail
        fail2

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed";

    let x = match cargo_test_result_parser(output) {
        IResult::Done(_, x) => x,
        _ => panic!("BOOM!"),
    };

    assert_eq!(
        x,
        vec![
            Suite {
                name: "target/debug/deps/docker_command-be014e20fbd07382".to_string(),
                state: "pass".to_string(),
                passed: 0,
                failed: 0,
                ignored: 0,
                measured: 0,
                total: 0,
                tests: vec![]
            },
            Suite {
                name: "target/debug/integration_test-d4fc68dd5824cbb9".to_string(),
                state: "fail".to_string(),
                passed: 1,
                failed: 2,
                ignored: 0,
                measured: 0,
                total: 3,
                tests: vec![
                    Test {
                        name: "fail".to_string(),
                        status: "fail".to_string(),
                        error: Some("thread \'fail\' panicked at \'assertion failed: `(left == right)` (left: `1`, right: `2`)\', tests/integration_test.rs:16".to_string())
                    },
                    Test {
                        name: "fail2".to_string(),
                        status: "fail".to_string(),
                        error: Some("thread \'fail2\' panicked at \'assertion failed: `(left == right)` (left: `3`, right: `2`)\', tests/integration_test.rs:22".to_string())
                    },
                    Test {
                        name: "it_runs_a_command".to_string(),
                        status: "pass".to_string(),
                        error: None
                    }
                ]
            }
        ]
    );
}

#[test]
fn test_success_run() {
    let output = b"   Compiling rustc-serialize v0.3.22
   Compiling toml v0.2.1
   Compiling pre-commit v0.5.2
   Compiling foo v0.1.0 (file:///foo)
    Finished debug [unoptimized + debuginfo] target(s) in 12.11 secs
     Running target/debug/deps/foo-5a7be5d1b9c8e0f6

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/integration_test-283604d1063344ba

running 1 test
test it_runs_a_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests foo

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out";

    assert_done(
        cargo_test_result_parser(output),
        vec![
            Suite {
                name: "target/debug/deps/foo-5a7be5d1b9c8e0f6".to_string(),
                state: "pass".to_string(),
                passed: 0,
                failed: 0,
                ignored: 0,
                measured: 0,
                total: 0,
                tests: vec![]
            },
            Suite {
                name: "target/debug/integration_test-283604d1063344ba".to_string(),
                state: "pass".to_string(),
                passed: 1,
                failed: 0,
                ignored: 0,
                measured: 0,
                total: 1,
                tests: vec![
                    Test {
                        name: "it_runs_a_command".to_string(),
                        status: "pass".to_string(),
                        error: None
                    }
                ]
            },
            Suite {
                name: "foo".to_string(),
                state: "pass".to_string(),
                passed: 0,
                failed: 0,
                ignored: 0,
                measured: 0,
                total: 0,
                tests: vec![]
            }
        ],
    );
}

#[test]
fn test_full_run() {
    let output = b"    Updating registry `https://github.com/rust-lang/crates.io-index`
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
     Running target/debug/deps/libzfs_sys-a797c24cd4b4a7ea

running 3 tests
test bindgen_test_layout_zpool_handle ... ok
test tests::open_close_handle ... ok
test tests::pool_search_import_list_export ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests libzfs-sys

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

    ";

    assert_done(
        cargo_test_result_parser(output),
        vec![
            Suite {
                name: "target/debug/deps/libzfs_sys-a797c24cd4b4a7ea".to_string(),
                state: "pass".to_string(),
                passed: 3,
                failed: 0,
                ignored: 0,
                measured: 0,
                total: 3,
                tests: vec![
                    Test {
                        name: "bindgen_test_layout_zpool_handle".to_string(),
                        status: "pass".to_string(),
                        error: None
                    },
                    Test {
                        name: "tests::open_close_handle".to_string(),
                        status: "pass".to_string(),
                        error: None
                    },
                    Test {
                        name: "tests::pool_search_import_list_export".to_string(),
                        status: "pass".to_string(),
                        error: None
                    }
                ]
            },
            Suite {
                name: "libzfs-sys".to_string(),
                state: "pass".to_string(),
                passed: 0,
                failed: 0,
                ignored: 0,
                measured: 0,
                total: 0,
                tests: vec![]
            }
        ],
    );

}

#[test]
pub fn compile_fail() {
    let output = b"    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading cargo-test-junit v0.6.2
  Installing cargo-test-junit v0.6.2
 Downloading test-to-vec v0.4.2
 Downloading nom v2.2.1
 Downloading clap v2.28.0
 Downloading sxd-document v0.2.4
 Downloading duct v0.4.0
 Downloading textwrap v0.9.0
 Downloading bitflags v1.0.1
 Downloading vec_map v0.8.0
 Downloading unicode-width v0.1.4
 Downloading strsim v0.6.0
 Downloading atty v0.2.3
 Downloading ansi_term v0.10.2
 Downloading peresil v0.3.0
 Downloading typed-arena v1.3.0
 Downloading libc v0.2.34
 Downloading crossbeam v0.3.0
   Compiling nom v2.2.1
   Compiling libc v0.2.34
   Compiling crossbeam v0.3.0
   Compiling unicode-width v0.1.4
   Compiling strsim v0.6.0
   Compiling bitflags v1.0.1
   Compiling vec_map v0.8.0
   Compiling peresil v0.3.0
   Compiling typed-arena v1.3.0
   Compiling ansi_term v0.10.2
   Compiling test-to-vec v0.4.2
   Compiling atty v0.2.3
   Compiling duct v0.4.0
   Compiling textwrap v0.9.0
   Compiling sxd-document v0.2.4
   Compiling clap v2.28.0
   Compiling cargo-test-junit v0.6.2
    Finished release [optimized] target(s) in 114.51 secs
  Installing /root/.cargo/bin/cargo-test-junit
    Updating git repository `https://github.com/jgrund/rust-libzfs.git`
 Downloading pkg-config v0.3.9
 Downloading bindgen v0.30.0
 Downloading env_logger v0.4.3
 Downloading peeking_take_while v0.1.2
 Downloading quasi v0.32.0
 Downloading cfg-if v0.1.2
 Downloading clap v2.27.1
 Downloading aster v0.41.0
 Downloading syntex_syntax v0.58.1
 Downloading regex v0.2.2
 Downloading lazy_static v0.2.11
 Downloading which v1.0.3
 Downloading clang-sys v0.19.0
 Downloading cexpr v0.2.2
 Downloading log v0.3.8
 Downloading memchr v1.0.2
 Downloading utf8-ranges v1.0.0
 Downloading thread_local v0.3.4
 Downloading aho-corasick v0.6.3
 Downloading regex-syntax v0.4.1
 Downloading libc v0.2.33
 Downloading unreachable v1.0.0
 Downloading void v1.0.2
 Downloading syntex_errors v0.58.1
 Downloading rustc-serialize v0.3.24
 Downloading bitflags v0.8.2
 Downloading syntex_pos v0.58.1
 Downloading unicode-xid v0.0.4
 Downloading term v0.4.6
 Downloading ansi_term v0.9.0
 Downloading bitflags v0.9.1
 Downloading libloading v0.4.2
 Downloading glob v0.2.11
 Downloading nom v3.2.1
 Downloading quasi_codegen v0.32.0
 Downloading syntex v0.58.1
 Downloading cstr-argument v0.0.2
   Compiling strsim v0.6.0
   Compiling unicode-xid v0.0.4
   Compiling glob v0.2.11
   Compiling log v0.3.8
   Compiling nvpair-sys v0.1.0 (https://github.com/jgrund/rust-libzfs.git?rev=get-values#470f3014)
   Compiling rustc-serialize v0.3.24
   Compiling vec_map v0.8.0
   Compiling cfg-if v0.1.2
   Compiling unicode-width v0.1.4
   Compiling libloading v0.4.2
   Compiling pkg-config v0.3.9
   Compiling lazy_static v0.2.11
   Compiling ansi_term v0.9.0
   Compiling peeking_take_while v0.1.2
   Compiling libc v0.2.33
   Compiling utf8-ranges v1.0.0
   Compiling term v0.4.6
   Compiling bitflags v0.8.2
   Compiling bitflags v0.9.1
   Compiling regex-syntax v0.4.1
   Compiling void v1.0.2
   Compiling clang-sys v0.19.0
   Compiling syntex_pos v0.58.1
   Compiling textwrap v0.9.0
   Compiling memchr v1.0.2
   Compiling atty v0.2.3
   Compiling which v1.0.3
   Compiling unreachable v1.0.0
   Compiling syntex_errors v0.58.1
   Compiling nom v3.2.1
   Compiling cstr-argument v0.0.2
   Compiling aho-corasick v0.6.3
   Compiling clap v2.27.1
   Compiling thread_local v0.3.4
   Compiling syntex_syntax v0.58.1
   Compiling cexpr v0.2.2
   Compiling nvpair v0.2.0 (https://github.com/jgrund/rust-libzfs.git?rev=get-values#470f3014)
   Compiling regex v0.2.2
   Compiling aster v0.41.0
   Compiling syntex v0.58.1
   Compiling quasi v0.32.0
   Compiling env_logger v0.4.3
   Compiling quasi_codegen v0.32.0
   Compiling bindgen v0.30.0
   Compiling libzfs-sys v0.1.0 (file:///vagrant/libzfs-sys)
   Compiling libzfs v0.1.0 (file:///vagrant/libzfs)
error[E0369]: binary operation `==` cannot be applied to type `std::result::Result<nvpair::NvData, std::io::Error>`
   --> libzfs/src/lib.rs:134:9
    |
134 |         assert_eq!(state, Ok(nvpair::NvData::Uint64(sys::pool_state::POOL_STATE_EXPORTED as u64)));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: an implementation of `std::cmp::PartialEq` might be missing for `std::result::Result<nvpair::NvData, std::io::Error>`
    = note: this error originates in a macro outside of the current crate

error: aborting due to previous error

error: Could not compile `libzfs`.

To learn more, run the command again with --verbose.
";

    assert_done(
        cargo_test_result_parser(output),
        vec![
            Suite {
                name: "unknown".to_string(),
                state: "fail".to_string(),
                passed: 0,
                failed: 1,
                ignored: 0,
                measured: 0,
                total: 1,
                tests: vec![
                    Test {
                        name: "compile failed".to_string(),
                        status: "fail".to_string(),
                        error: Some("binary operation `==` cannot be applied to type `std::result::Result<nvpair::NvData, std::io::Error>`
   --> libzfs/src/lib.rs:134:9
    |
134 |         assert_eq!(state, Ok(nvpair::NvData::Uint64(sys::pool_state::POOL_STATE_EXPORTED as u64)));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: an implementation of `std::cmp::PartialEq` might be missing for `std::result::Result<nvpair::NvData, std::io::Error>`
    = note: this error originates in a macro outside of the current crate
".to_string())
                    },
                ]
            }
        ],
    );
}

#[test]
fn test_success_on_windows_rust_1_39() {
    let output = b"    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling cfg-if v0.1.10
   Compiling autocfg v0.1.7
   Compiling winapi v0.3.8
   Compiling lazy_static v1.4.0
   Compiling semver-parser v0.7.0
   Compiling libc v0.2.65
   Compiling byteorder v1.3.2
   Compiling maybe-uninit v2.0.0
   Compiling futures v0.1.29
   Compiling log v0.4.8
   Compiling proc-macro2 v1.0.6                                                                                                          
   Compiling iovec v0.1.4                                                                                                                
   Compiling unicode-xid v0.2.0                                                                                                          
   Compiling syn v1.0.8                                                                                                                  
   Compiling winapi-build v0.1.1                                                                                                         
   Compiling scopeguard v1.0.0                                                                                                           
   Compiling cc v1.0.47                                                                                                                  
   Compiling fnv v1.0.6                                                                                                                  
   Compiling winapi v0.2.8                                                                                                               
   Compiling slab v0.4.2                                                                                                                 
   Compiling proc-macro2 v0.4.30                                                                                                         
   Compiling unicode-xid v0.1.0                                                                                                          
   Compiling rand_core v0.4.2                                                                                                            
   Compiling memchr v2.2.1                                                                                                               
   Compiling syn v0.15.44                                                                                                                
   Compiling matches v0.1.8                                                                                                              
   Compiling getrandom v0.1.13                                                                                                           
   Compiling regex-syntax v0.6.12                                                                                                        
   Compiling itoa v0.4.4                                                                                                                 
   Compiling ppv-lite86 v0.2.6                                                                                                           
   Compiling failure_derive v0.1.6                                                                                                       
   Compiling serde v1.0.102                                                                                                              
   Compiling rustc-demangle v0.1.16                                                                                                      
   Compiling ipconfig v0.2.1                                                                                                             
   Compiling ryu v1.0.2                                                                                                                  
   Compiling percent-encoding v1.0.1                                                                                                     
   Compiling copyless v0.1.4
   Compiling quick-error v1.2.2                                                                                                          
   Compiling linked-hash-map v0.5.2                                                                                                      
   Compiling widestring v0.4.0                                                                                                           
   Compiling crc32fast v1.2.0                                                                                                            
   Compiling either v1.5.3                                                                                                               
   Compiling percent-encoding v2.1.0                                                                                                     
   Compiling slog v2.5.2                                                                                                                 
   Compiling encoding_rs v0.8.20                                                                                                         
   Compiling httparse v1.3.4                                                                                                             
   Compiling bitflags v1.2.1                                                                                                             
   Compiling dtoa v0.4.4                                                                                                                 
   Compiling language-tags v0.2.2                                                                                                        
   Compiling sha1 v0.6.0                                                                                                                 
   Compiling mime v0.3.14                                                                                                                
   Compiling protobuf v2.8.1                                                                                                             
   Compiling arc-swap v0.4.3                                                                                                             
   Compiling prometheus v0.7.0                                                                                                           
   Compiling hashbrown v0.5.0                                                                                                            
   Compiling spin v0.5.2                                                                                                                 
   Compiling crossbeam-utils v0.6.6                                                                                                      
   Compiling thread_local v0.3.6                                                                                                         
   Compiling semver v0.9.0                                                                                                               
   Compiling rand_pcg v0.1.2                                                                                                             
   Compiling rand_chacha v0.1.1                                                                                                          
   Compiling crossbeam-utils v0.7.0                                                                                                      
   Compiling rand v0.6.5                                                                                                                 
   Compiling num-traits v0.2.8                                                                                                           
   Compiling num-integer v0.1.41                                                                                                         
   Compiling indexmap v1.3.0                                                                                                             
   Compiling hashbrown v0.6.3                                                                                                            
   Compiling crossbeam-epoch v0.8.0                                                                                                      
   Compiling lock_api v0.3.1                                                                                                             
   Compiling ws2_32-sys v0.2.1                                                                                                           
   Compiling kernel32-sys v0.2.2                                                                                                         
   Compiling tokio-sync v0.1.7                                                                                                           
   Compiling actix-service v0.4.2                                                                                                        
   Compiling rand_core v0.3.1                                                                                                            
   Compiling unicode-bidi v0.3.4                                                                                                         
   Compiling backtrace-sys v0.1.32                                                                                                       
   Compiling miniz-sys v0.1.12                                                                                                           
   Compiling brotli-sys v0.3.2                                                                                                           
   Compiling c2-chacha v0.2.3                                                                                                            
   Compiling lru-cache v0.1.2                                                                                                            
   Compiling tokio-executor v0.1.8                                                                                                       
   Compiling rustc_version v0.2.3                                                                                                        
   Compiling rand_isaac v0.1.1                                                                                                           
   Compiling rand_hc v0.1.0                                                                                                              
   Compiling rand_xorshift v0.1.1                                                                                                        
   Compiling tokio-timer v0.2.11                                                                                                         
   Compiling tokio-current-thread v0.1.6                                                                                                 
   Compiling parking_lot_core v0.6.2                                                                                                     
   Compiling parking_lot v0.9.0                                                                                                          
   Compiling memoffset v0.5.3                                                                                                            
   Compiling num_cpus v1.11.0                                                                                                            
   Compiling smallvec v0.6.13                                                                                                            
   Compiling bytes v0.4.12                                                                                                               
   Compiling base64 v0.10.1                                                                                                              
   Compiling quote v1.0.2                                                                                                                
   Compiling quote v0.6.13                                                                                                               
   Compiling aho-corasick v0.7.6                                                                                                         
   Compiling rand_core v0.5.1                                                                                                            
   Compiling net2 v0.2.33                                                                                                                
   Compiling rand_jitter v0.1.4                                                                                                          
   Compiling socket2 v0.3.11                                                                                                             
   Compiling rand_os v0.1.3                                                                                                              
   Compiling winutil v0.1.1                                                                                                              
   Compiling winreg v0.6.2                                                                                                               
   Compiling time v0.1.42                                                                                                                
   Compiling slog-scope v4.3.0                                                                                                           
   Compiling threadpool v1.7.1                                                                                                           
   Compiling crossbeam-queue v0.2.0                                                                                                      
   Compiling crossbeam-channel v0.4.0                                                                                                    
   Compiling unicode-normalization v0.1.9                                                                                                
   Compiling tokio-io v0.1.12                                                                                                            
   Compiling http v0.1.19                                                                                                                
   Compiling string v0.2.1                                                                                                               
   Compiling backtrace v0.3.40                                                                                                           
   Compiling rand_chacha v0.2.1                                                                                                          
   Compiling miow v0.2.1                                                                                                                 
   Compiling regex v1.3.1                                                                                                                
   Compiling hostname v0.1.5                                                                                                             
   Compiling flate2 v1.0.12                                                                                                              
   Compiling brotli2 v0.3.2                                                                                                              
   Compiling idna v0.1.5                                                                                                                 
   Compiling idna v0.2.0                                                                                                                 
   Compiling tokio-codec v0.1.1                                                                                                          
   Compiling chrono v0.4.9                                                                                                               
   Compiling h2 v0.1.26                                                                                                                  
   Compiling rand v0.7.2                                                                                                                 
   Compiling mio v0.6.19                                                                                                                 
   Compiling enum-as-inner v0.2.1                                                                                                        
   Compiling resolv-conf v0.6.2                                                                                                          
   Compiling synstructure v0.12.2                                                                                                        
   Compiling url v1.7.2                                                                                                                  
   Compiling url v2.1.0                                                                                                                  
   Compiling actix-codec v0.1.2                                                                                                          
   Compiling derive_more v0.15.0                                                                                                         
   Compiling serde_derive v1.0.102                                                                                                       
   Compiling proc-macro-hack v0.5.11                                                                                                     
   Compiling actix-web-codegen v0.1.3                                                                                                    
   Compiling crossbeam-deque v0.7.2                                                                                                      
   Compiling tokio-reactor v0.1.10                                                                                                       
   Compiling actix-utils v0.4.7                                                                                                          
   Compiling const-random-macro v0.1.6                                                                                                   
   Compiling crossbeam v0.7.3                                                                                                            
   Compiling tokio-tcp v0.1.3                                                                                                            
   Compiling tokio-udp v0.1.5                                                                                                            
   Compiling tokio-signal v0.2.7                                                                                                         
   Compiling const-random v0.1.6                                                                                                         
   Compiling slog-stdlog v4.0.0                                                                                                          
   Compiling actix-server-config v0.1.2                                                                                                  
   Compiling failure v0.1.6                                                                                                              
   Compiling actix-threadpool v0.1.2                                                                                                     
   Compiling ahash v0.2.17                                                                                                               
   Compiling trust-dns-proto v0.7.4                                                                                                      
   Compiling actix-rt v0.2.5                                                                                                             
   Compiling actix-server v0.6.1                                                                                                         
   Compiling actix-testing v0.1.0                                                                                                        
   Compiling trust-dns-resolver v0.11.1                                                                                                  
   Compiling actix-connect v0.2.5                                                                                                        
   Compiling serde_json v1.0.41                                                                                                          
   Compiling serde_urlencoded v0.6.1
   Compiling actix-router v0.1.5
   Compiling actix-http v0.2.11                                                                                                          
   Compiling slog-json v2.3.0
   Compiling awc v0.2.8                                                                                                                  
   Compiling actix-web v1.0.8                                                                                                            
   Compiling app v0.1.0 (C:\\Temp\\Rust)                                               
    Finished dev [unoptimized + debuginfo] target(s) in 6m 22s                                                                           
     Running target\\debug\\deps\\app-622eefdc86aa5319.exe

running 1 test
test router::handlers::tests::test_success ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
";

    assert_done(
        cargo_test_result_parser(output),
        vec![Suite {
            name: "target\\debug\\deps\\app-622eefdc86aa5319.exe".to_string(),
            state: "pass".to_string(),
            passed: 1,
            failed: 0,
            ignored: 0,
            measured: 0,
            total: 1,
            tests: vec![Test {
                name: "router::handlers::tests::test_success".to_string(),
                status: "pass".to_string(),
                error: None,
            }],
        }],
    );
}
