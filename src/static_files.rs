extern crate rand;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::response::Redirect;
use dotenv::dotenv;
use std::env;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::http::{Cookie, Cookies};
use std::collections::HashMap;
// use rocket::http::hyper::header::Headers;
// use rocket::http::hyper::header::Authorization;
// use hyper::header::{Headers, Authorization};
use tokio;

// extern crate mongodb;
extern crate base64_url;
extern crate reqwest;
extern crate serde;
// use reqwest::Client;
// use serde::Deserialize;
// use mongodb::{Client, options::ClientOptions};

#[derive(Serialize, Deserialize, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u32,
    refresh_token: String, 
}


#[get("/index")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/<file..>", rank = 5)]
pub fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

#[get("/login")]
pub fn login(mut cookies: Cookies) -> rocket::response::Redirect {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let response_type = "code";
    let redirect_uri= "http://localhost:8001/api/v1/callback";

    let mut rng = thread_rng();
    let state: String =  iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(16)
        .collect();
    cookies.add(Cookie::new("state", state.clone()));
    
    let redirect = Redirect::to(
        format!("https://accounts.spotify.com/authorize?client_id={}&response_type={}&redirect_uri={}&state={}", 
            client_id, 
            response_type, 
            redirect_uri,
            state
        )
    );
    // println!("{}", redirect.to_string());
    redirect
}


#[get("/callback?<code>&<state>")]
pub fn callback(code: String, state: String, cookies: Cookies) {
    dotenv().ok();

    let cookie_state = cookies.get("state").map(|c| c.value()).unwrap();
    // println!("{}", code);
    
    if  cookie_state != state || state.is_empty() {
        println!("bad state");
    }
    
    let mut map = HashMap::new();
    map.insert("grant_type", "authorization_code");
    map.insert("code", &code);
    map.insert("redirect_uri", "http://localhost:8001/api/v1/callback");
    
    // let mut headers = Headers::new();
    let mut authcode = "Basic ".to_owned(); 
    let mut secret = env::var("CLIENT_ID").unwrap().to_owned();
    secret.push_str(":");
    secret.push_str(&env::var("CLIENT_SECRET").unwrap());
    let encoded = base64_url::encode(&secret);
    authcode.push_str(&encoded);
    // println!("{}", authcode);
    // println!("calling funcition");
    let res = auth_token_request(map, authcode).unwrap();
    
    println!{"{:#?}", res};
    // println!("function called");

}

#[tokio::main]
async fn auth_token_request(map: HashMap<&str, &str>, header: String) -> Result<Token, reqwest::Error>{
    let res: Token = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", header)
        .form(&map)
        .send()
        .await?
        .json()
        .await?;

    // let client = mongodb::Client::with_uri_str("mongodb://localhost:27017/").await?;
    Ok(res)
}