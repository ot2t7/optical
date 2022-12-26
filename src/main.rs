use std::io::{Cursor, Read};
use wasabi_leb128::ReadLeb128;

use anyhow::Result;

mod listener;
mod packet_defs;
mod packet_format;

#[tokio::main]
async fn main() -> Result<()> {
    /*
    let vec = vec![1u8, 0b00001101, 0b01110011, 2, 3, 4, 5];
    let buf = &mut Cursor::new(vec);
    read_something(buf)?;
    read_leb(buf)?;
    read_something(buf)?;
    read_something(buf)?;

    return Ok(());
    */

    listener::start().await?;
    return Ok(());
}

fn read_something(buf: &mut Cursor<Vec<u8>>) -> Result<()> {
    let mut byte = [0u8];
    buf.read_exact(&mut byte)?;
    println!("{}", byte[0]);
    return Ok(());
}

fn read_leb(buf: &mut Cursor<Vec<u8>>) -> Result<()> {
    let val: i32 = buf.read_leb128()?.0;
    println!("{}", val);
    return Ok(());
}
