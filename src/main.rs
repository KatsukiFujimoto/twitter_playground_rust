use dotenv::dotenv;
use std::env;
use twitter_v2::{authorization::BearerToken, TwitterApi};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bearer_token = env::var("TWITTER_BEARER_TOKEN").expect("TWITTER_BEARER_TOKEN must be set");
    let auth = BearerToken::new(bearer_token);
    let res = TwitterApi::new(auth)
        .get_tweets_search_recent("technology")
        .max_results(10)
        .send()
        .await
        .expect("there was something wrong");
    println!("data: {:?}", res.data);
    println!("meta: {:?}", res.meta);
    println!("includes: {:?}", res.includes);
    println!("errors: {:?}", res.errors);
}
