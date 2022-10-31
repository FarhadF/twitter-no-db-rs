use actix_web::web::{Data, Json};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tweets {
    pub tweets: Vec<Tweet>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tweet {
    pub id: String,
    pub message: String,
    pub created_at: DateTime<Utc>, //todo: likes
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello {name}!"))
}

#[get("/tweets")]
async fn list_tweets(req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<Tweets>>>().unwrap();
    let my_data = data.lock().unwrap();
    let mut sorted = my_data.tweets.to_vec();
    sorted.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    HttpResponse::Ok().json(sorted)
}

#[post("/tweet")]
async fn tweet(tweet_req: Json<TweetRequest>, req: HttpRequest) -> HttpResponse {
    let data = req.app_data::<Data<Mutex<Tweets>>>().unwrap();
    let mut my_data = data.lock().unwrap();
    let tweet = tweet_req.to_tweet();
    match tweet {
        Some(tweet) => my_data.tweets.push(tweet),
        None => (),
    }
    HttpResponse::Created().json(my_data.tweets.last())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Data::new(Mutex::new(Tweets { tweets: vec![] }));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(greet)
            .service(list_tweets)
            .service(tweet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
