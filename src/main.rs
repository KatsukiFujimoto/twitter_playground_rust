use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    println!(
        "TWITTER_BEARER_TOKEN: {}",
        env::var("TWITTER_BEARER_TOKEN").expect("TWITTER_BEARER_TOKEN must be set")
    );
}
