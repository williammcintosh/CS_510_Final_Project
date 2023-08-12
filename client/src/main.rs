// use reqwest::Client;

mod api_calls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    api_calls::post_user("new@cl.ent", "pass1234", "pass1234").await?;

    // api_calls::get_all_apods().await?;

    // api_calls::get_all_questions("1").await?;

    Ok(())
}

