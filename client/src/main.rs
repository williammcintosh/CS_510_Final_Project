// use reqwest::Client;

mod api_calls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    api_calls::get_all_apods().await?;

    // api_calls::get_all_questions("1").await?;

    Ok(())
}

