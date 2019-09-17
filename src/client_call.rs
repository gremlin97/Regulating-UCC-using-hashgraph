use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::io::{self, Write, Read};

use hyper::rt::{self, Future, Stream};


#[derive(Serialize, Deserialize)]
pub struct SplitSet {
    pub share_rtm: String,
    pub share_oap: String,
    pub share_ir: String,
    pub share_tap : String
}

fn get_request(uri : String) -> String {
    let mut response = reqwest::get(&uri).expect("failed to send request");
    println!("Response Status: {}", response.status());

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("failed to read response");
    buf

}

pub fn post_request(split_set: &SplitSet, dest : String ) -> bool {
    let res = json!( {
            "share_ir": split_set.share_ir,
            "share_oap": split_set.share_oap,
            "share_rtm": split_set.share_rtm,
            "share_tap": split_set.share_tap
    });

    let uri;
    if dest == "OAP" {
        uri = "http://localhost:8000/oap"
    } else {
        uri = "http://localhost:8000/tap"
    }
    let client = reqwest::Client::new();
    let mut response = client.post(uri)
        .json(&res)
        .send()
        .expect("Failed to send request");
    assert_eq!(response.status(), 200);

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);

    if response.status() == 200 {
        true
    } else {
        false
    }
}