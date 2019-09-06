#![recursion_limit = "512"]
#![allow(warnings)]

#[macro_use]
extern crate yew;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate stdweb;

pub mod app;
pub mod components;
pub mod stdweb_calls;
