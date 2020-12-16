/*!
* the reason that I didn't use trello-rs, is because it is quite limited in configurability
* the same for the trello crate
*/
#![allow(incomplete_features)]
#![feature(let_chains)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate err_derive;
#[macro_use]
extern crate serde_derive;
pub mod bot;
pub mod commands;
pub mod trellobot;
