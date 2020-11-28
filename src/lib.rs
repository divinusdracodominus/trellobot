#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate err_derive;
#[macro_use]
extern crate serde_derive;
pub mod bot;
pub mod command;
pub mod error;
pub mod trellobot;
