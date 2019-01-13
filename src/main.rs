mod reqwest_client;

use crate::reqwest_client::ReqwestClient;

fn main() {
    let mut client = ReqwestClient::new("test").unwrap();

    let text = client
        .get("https://www.google.com/")
        .unwrap();

    println!("{}", text.len());
}
