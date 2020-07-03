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
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::{JsonValue};


use std::collections::HashMap;
// use rocket::http::hyper::header::Headers;
// use rocket::http::hyper::header::Authorization;
// use hyper::header::{Headers, Authorization};
use tokio;



use crate::spotify::structs;

extern crate mongodb;
extern crate base64_url;
extern crate reqwest;
// use reqwest::Client;
use serde::Deserialize;

use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;





#[derive(Serialize, Deserialize, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u32,
    refresh_token: String, 
}

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
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
pub fn callback(code: String, state: String, cookies: Cookies) -> rocket::response::Redirect {
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






    println!("calling funcition");
    let res = auth_token_request(map, authcode).unwrap();
    // let res2 = res.clone();
    let client = Client::connect("localhost", 27017)
    .expect("Failed to initialize standalone client.");
    let coll = client.db("test").collection("users");
    let doc = doc! {
        "name": "Jaws",
        "access_token": res.access_token,
        "token_type": res.token_type,
        "scope": res.scope,
        "expires_in": res.expires_in,
        "refresh_token": res.refresh_token,
    };


    coll.insert_one(doc.clone(), None).ok().expect("Failed to insert doc.");
    
    println!("function called");
    // ApiResponse {
    //     json: json!(doc),
    //     status: Status::Ok,
    // }
    let redirect = Redirect::to(
        format!("http://localhost:8080/home" 
        )
    );
    // println!("{}", redirect.to_string());
    redirect
}

#[tokio::main]
async fn auth_token_request(map: HashMap<&str, &str>, header: String) -> Result<Token, reqwest::Error> {
    let res: Token = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", header)
        .header("Access-Control-Allow-Origin", "*")
        .form(&map)
        .send()
        .await?
        .json()
        .await?;
    
    // let client = mongodb::Client::with_uri_str("mongodb://localhost:27017/").await?;
    Ok(res)
}

#[get("/github")]
pub fn github() -> String {
    dummy_api_call().unwrap()
    
}


#[tokio::main]
async fn dummy_api_call() -> Result<String, reqwest::Error> {
    let client = Client::connect("localhost", 27017)
    .expect("Failed to initialize standalone client.");
    let coll = client.db("test").collection("users");

    let person =  coll.find_one(Some(doc! { "_id": bson::oid::ObjectId::with_string("5efca1fa346362d124256263").unwrap() }), None)
    .expect("Document not found");
    println!("{:?}", person);

    let user = "6oMuImdp5ZcFhWP0ESe6mG";
    // let access_token = "Bearer ".to_owned() + cookies.get("access_token").map(|c| c.value()).unwrap();
    let mut access_token = "dd".to_owned();
    match person {
        Some(doc) => match doc.get("access_token") {
            Some(&Bson::String(ref token)) => access_token = "Bearer ".to_owned() + token,
            _ => panic!("Bruh!"),
        },
        None => panic!("Server returned no results!"),
    }

    let request_url = format!("https://api.spotify.com/v1/artists/{}", user);

    println!("{}", request_url);
    println!("access token {}", access_token);
    let client = reqwest::Client::new();
    let res = client
        .get(&request_url)
        .header("Authorization", access_token)
        .send()
        .await?
        .text()
        .await?;

    println!("response: {:#?}", res);

    Ok(res)
}

// #[tokio::main]
// async fn get_user_spotify(access_token: String) -> Result<spotify::User, reqwest::Error> {

//     // let access_token = "Bearer ".to_owned() + cookies.get("access_token").map(|c| c.value()).unwrap();
//     let token = "Bearer".to_owned() + &access_token;

//     let request_url = format!("https://api.spotify.com/v1/me");

//     println!("{}", request_url);
//     println!("access token {}", token);
//     let client = reqwest::Client::new();
//     let res: spotify::User = client
//         .get(&request_url)
//         .header("Authorization", token)
//         .send()
//         .await?
//         .json()
//         .await?;

//     println!("response: {:#?}", res);

//     Ok(res)
// }