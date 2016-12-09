extern crate rustc_serialize as rustc_serialize;
extern crate toml;
extern crate egg_mode;

use std::convert::AsRef;
use std::clone::Clone;
use std::io::prelude::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct AppConfig {
    consumer_key: String,
    consumer_secret: String,
}

pub struct Twitter_Authorizer<'a> {
    consumer: egg_mode::Token<'a>,
    request_token_pool: HashMap<i64, egg_mode::Token<'a>>,
}

pub fn new<'a> (config: &str) -> Twitter_Authorizer<'a> {
    let mut f = File::open(config).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let decoded: AppConfig = toml::decode_str(&s).unwrap();
    Twitter_Authorizer {
        consumer: egg_mode::Token::new(decoded.consumer_key, decoded.consumer_secret),
        request_token_pool: HashMap::new()
    }
}

impl<'a> Twitter_Authorizer<'a> {
    pub fn access_token (&self, oauth_verifier: String) -> String {
        let request = &self.request_token_pool.get(&1).unwrap();
        let (token, id, name) = egg_mode::access_token(&self.consumer, request, oauth_verifier).unwrap();
        println!("id: {}, name: {}", id, name);
        return format!("id: {}, name: {}", id, name);
    }
    pub fn generate_authorize_url (&mut self) -> String {
        let mut request = egg_mode::request_token(&self.consumer, "http://localhost:6767/sign-in/callback/").unwrap();
        let url = egg_mode::authenticate_url(&request);
        &self.request_token_pool.insert(1, request);
        return url;
    }
}

