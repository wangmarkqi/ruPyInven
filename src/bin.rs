#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod inven;
pub mod common;
fn main(){
    inven::everyday::test();
}
