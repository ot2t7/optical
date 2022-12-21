use anyhow::Result;

mod listener;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    /*
    let mut buf = vec![];
    buf.extend_from_slice(&vec![0u8; 1024 * 1024 * 1024 * 4]);
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    println!("Dropping the first half of the buf..");
    let second_half = buf.split_off(1024 * 1024 * 1024 * 2);
    let first_half = buf.clone();
    //drop(buf)
    println!("Done");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    */
    listener::start().await?;
    return Ok(());
}
