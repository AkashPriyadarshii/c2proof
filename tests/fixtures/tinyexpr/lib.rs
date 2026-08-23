#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]


extern crate libc;
pub mod src {
pub mod benchmark;
pub mod example;
pub mod example2;
pub mod example3;
pub mod repl;
pub mod smoke;
pub mod tinyexpr;
} // mod src
