use std::error::Error;

use reqwest;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut resp = reqwest::blocking::get(url)?;

    let content = resp.text()?;
    print!("{}", content);

    Ok(())
}