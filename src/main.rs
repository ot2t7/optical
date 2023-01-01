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
