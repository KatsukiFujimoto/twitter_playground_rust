use csv::Writer;
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
    let mut csv = Writer::from_path("hoge.csv").expect("could'nt set up csv writer");
    if let Some(data) = res.into_data() {
        csv.write_record(["tweet_id", "contents"]).expect("couldn't write to csv");
        for tweet in &data {
            csv.write_record([tweet.id.to_string(), tweet.text.clone()]).expect("couldn't write to csv");
        }
    }
}
