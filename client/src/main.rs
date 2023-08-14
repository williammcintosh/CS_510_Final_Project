// use reqwest::Client;

mod api_calls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // api_calls::get_all_apods().await?;

    // api_calls::post_user("first@site.com", "1234qwer", "1234qwer").await?;
    // api_calls::post_user("second@site.com", "1234qwer", "1234qwer").await?;
    //
    // api_calls::post_new_question(
    //     "Monster Solar Prominence",
    //     "2023-08-01",
    //     "The monsters that live on the Sun are not like us.",
    //     "https://apod.nasa.gov/apod/image/2308/SunMonster_Wenz_960.jpg",
    // ).await?;
    //
    // api_calls::post_new_comment("I didn't know sun monster's existed!", 1, 1).await?;
    // api_calls::post_new_comment("Such amazing sun flares!", 1, 2).await?;
    // api_calls::post_new_comment("Such smokiness!", 2, 2).await?;

    api_calls::post_new_favorite(1, 1).await?;
    api_calls::post_new_favorite(1, 3).await?;
    api_calls::post_new_favorite(2, 2).await?;

    Ok(())
}

