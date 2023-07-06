#![allow(unused)]

mod decode;
mod encode;
mod types;

use crate::encode::Encodable;
use crate::types::Result;

fn main() -> Result<()> {
    let v = decode::decode("^'i3'<'i3'<'sh''i7''sokay!'>>$".to_string())?;
    dbg!(&v);
    println!("{}", v.encode());

    Ok(())
}
