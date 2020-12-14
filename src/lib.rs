#![feature(let_chains)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate err_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
pub mod bot;
pub mod error;
pub mod trellobot;
pub mod commands;