#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate secsp_syntax;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
});
