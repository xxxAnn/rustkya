#![allow(unused)]

mod decode;
mod encode;
mod types;

use crate::encode::Encodable;

fn main() {
    let (v, u) = decode::decode("^'i3'<'i3'<'sh''i7''sokay!'>>$".to_string());
    dbg!(v.clone());
    println!("{}", v.encode());
}
