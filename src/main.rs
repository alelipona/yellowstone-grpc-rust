mod common;
mod subscribe_logs;
mod subscribe_tx;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Hello, world!");
}
