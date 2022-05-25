#![allow(unused)]

mod decode;

fn main() {
    let (v, u) = decode::decode("^'i3'<'i3'<'sh'>>$".to_string());
    dbg!(v);
}
