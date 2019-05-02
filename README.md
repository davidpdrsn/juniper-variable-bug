This repo is for reproducing a bug in [Juniper][].

Run `cargo test` to reproduce the error:

```
$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running target/debug/deps/juniper_variable_bug-b0090c9e420264b7

running 2 tests
test test::query_with_null ... ok
test test::query_without_setting_variable ... FAILED

failures:

---- test::query_without_setting_variable stdout ----
thread 'test::query_without_setting_variable' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    test::query_without_setting_variable

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--bin juniper-variable-bug'
```

Tested on `rustc 1.34.1 (fc50f328b 2019-04-24)`

[Juniper]: https://github.com/graphql-rust/juniper
