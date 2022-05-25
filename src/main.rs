#![allow(unused)]

mod decode;
mod types;

fn main() {
    let (v, u) = decode::decode("^'i3'<'i3'<'sh''i7''sokay!'>>$".to_string());
    dbg!(v);
}
