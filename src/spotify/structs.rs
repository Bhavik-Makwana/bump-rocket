extern crate serde;
// use reqwest::Client;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    country: String,
    display_name: String,
    email: String,
    external_urls: ExternalUrls,
    followers: Followers,
    href: String,
    id: String,
    images: ImageObject,
    product: String,
    types: String,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    key: String,
    value: String    
}

#[derive(Serialize, Deserialize, Debug)]
struct Followers {
    href: String,
    total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageObject {
    height: u32,
    url: String,
    width: u32,
}