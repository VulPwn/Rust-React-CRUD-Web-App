extern crate reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://maps.googleapis.com/maps/api/place/autocomplete/json?input='5064 Twin'&components=country:us&key=AIzaSyAqpUzPe_64qht8DYd472JzKgOprlCnh2s")
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("body = {:?}", body);
    Ok(())
}
