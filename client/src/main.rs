// use reqwest::Client;

mod api_calls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    api_calls::post_user("first@site.com", "pass1234", "pass1234").await?;
    api_calls::post_user("second@site.com", "pass1234", "pass1234").await?;
    api_calls::post_favorite("1", "1").await?; //first user's favorite is the first apod
    api_calls::post_favorite("1", "3").await?; //first user's favorite is the third apod
    api_calls::post_favorite("2", "2").await?; //second user's favorite is the second apod

    // api_calls::get_all_apods().await?;

    // api_calls::get_all_questions("1").await?;

    Ok(())
}

