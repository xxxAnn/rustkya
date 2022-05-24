#![allow(unused)]

use std::collections::HashMap;

mod decode;

#[derive(Debug)]
pub enum Decoded {
    Str(String),
    Num(u64),
    Dict(HashMap<Box<Decoded>, Box<Decoded>>),
    List(Vec<Box<Decoded>>)
}

fn main() {
    let (v, u) = decode::decode("<'i3''shey'>".to_string());
    dbg!(v);
}
