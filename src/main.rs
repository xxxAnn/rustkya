#![allow(unused)]

mod decode;

fn main() {
    let (v, u) = decode::decode("<'i3''shey'>".to_string());
    dbg!(v);
}
