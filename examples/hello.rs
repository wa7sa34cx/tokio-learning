use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // let s = "hello".into();

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    // if let Some(val) = result.unwrap() {
    //     println!("The result is {:?}", str::from_utf8(val));
    // }

    println!("got value from the server; result={:?}", result);

    Ok(())
}
