use dotenv::dotenv;
use reqwest::Client;
use std::env;
use serde_json::json;

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

pub async fn post_user(
    email: &str,
    password: &str,
    confirm_password: &str,
) -> anyhow::Result<()> {
    let api_port = get_api_port()?;
    // Create a reqwest client
    let client = Client::new();

    // `serde_json::Value`
    let body_json = json!({
        "email": email,
        "password": password,
        "confirm_password": confirm_password,
    });

    let url = format!("http://localhost:{}", api_port) + "/users";
    println!("URL = {}", url);

    // Same as GET, but makes a POST request with appropriate header
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body_json.to_string())
        .send()
        .await?;

    let body = res.text().await?;
    println!("POST user: {}", body);

    Ok(())
}

pub async fn post_favorite(
    user_id: &str,
    apod_id: &str,
) -> anyhow::Result<()> {
    let api_port = get_api_port()?;
    // Create a reqwest client
    let client = Client::new();

    let uid_int: i32 = user_id.parse().unwrap();
    let aid_int: i32 = apod_id.parse().unwrap();

    // `serde_json::Value`
    let body_json = json!({
        "user_id": uid_int,
        "apod_id": aid_int,
    });

    let url = format!("http://localhost:{}", api_port) + "/favorite";
    println!("URL = {}", url);

    // Same as GET, but makes a POST request with appropriate header
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body_json.to_string())
        .send()
        .await?;

    let body = res.text().await?;
    println!("POST user: {}", body);

    Ok(())
}