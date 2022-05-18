use std::{collections::HashMap, time::Duration};

const USERNAME: &str = "";
const PASSWORD: &str = "";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("login.rs")
        .connect_timeout(Duration::from_secs(8))
        .build()?;

    let req = client
        .get("http://172.29.1.3:8728/ip/hotspot/active/getall")
        .basic_auth(USERNAME, Some(PASSWORD))
        .query(&[("user", "parham.alvani")])
        .build()?;

    let resp = client.execute(req)?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
