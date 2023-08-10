use dotenv::dotenv;
use reqwest::Client;
use std::env;

// Gets the api port from the .env file
fn get_api_port() -> Result<String, env::VarError> {
    dotenv().ok(); // Load the .env file
    let api_port = env::var("API_PORT")?; // Get the value of API_PORT from the .env file
    Ok(api_port)
}

pub async fn get_all_apods() -> anyhow::Result<()> {
    let api_port = get_api_port()?;
    // Create a reqwest client
    let client = Client::new();
    let url = format!("http://localhost:{}", api_port) + "/apods";
    println!("URL = {}", url);
    // Make a GET HTTP request to our backend's /example route
    let res = client
        .get(url)
        .send()
        .await?;

    // Get the response from backend's data
    let body = res.text().await?;

    // Print out that response
    println!("GET All Apods:\n{}", body);

    Ok(())
}