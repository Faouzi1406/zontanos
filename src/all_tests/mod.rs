//! This directory contains all tests for all components of the compiler
//!
//! This means that any part of the compiler that gets added should have a test.
//!
//! # Where to add my tests? :  
//!
//! lexer: if you write a adition to the lexer you should include a test in the [`lexer_tests.rs`] file

#[cfg(test)]
pub mod lexer_tests;
#[cfg(test)]
mod parser_tests;

#[macro_export]
macro_rules! panic_test {
    ($test_desc: expr, $msg: expr) => {
        panic!(
            "[TEST FAILED; ] name = {:#?} message = {:#?} ",
            $test_desc, $msg
        );
    };
}
