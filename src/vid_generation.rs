#[warn(dead_code)]
use rocket_contrib::json::{Json, JsonValue};
use shamir::SecretData;
use base64;
use std::io::{self, Write, Read};

use hyper::rt::{self, Future, Stream};
use std::collections::HashMap;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Result;
//use client_call::post_request;
//use client_call::SplitSet;


//#[derive(Serialize, Deserialize)]
//pub struct SplitSet {
//    pub share_rtm: String,
//    pub share_oap: String,
//    pub share_ir: String,
//    pub share_tap : String
//}
///splits the phone number
/// out of 3 are needed to open the number
/// number : phone number
/// threshold : min needed to open number
/// returns json of base64 splits
#[get("/<number>/<threshold>")]
pub fn generate_splits(number: String, threshold: u8) -> JsonValue {
    let number_to_split = SecretData::with_secret(&number, threshold);

    let share_oap = number_to_split.get_share(1);
    let share_tap = number_to_split.get_share(2);
    let share_ir = number_to_split.get_share(3);
    let share_rtm = number_to_split.get_share(4);

    let encoding_share_rtm: String = base64::encode(&share_rtm);
    let encoding_share_ir: String = base64::encode(&share_ir);
    let encoding_share_oap: String = base64::encode(&share_oap);
    let encoding_share_tap: String = base64::encode(&share_tap);

    ///loop around to check until
    /// if contains '/' or '+'
    /// send data to requested parties
    /// send whole packet

    let response = json!( {
        "share_rtm": encoding_share_rtm,
        "share_oap": encoding_share_ir,
        "share_ir": encoding_share_oap,
        "share_tap" : encoding_share_tap
    });
    let split_set = crate::client_call::SplitSet{
        share_rtm: encoding_share_rtm,
        share_oap: encoding_share_oap,
        share_ir: encoding_share_ir,
        share_tap : encoding_share_tap
    };

    println!("Initiating call from RTM to OAP");
    crate::client_call::post_request(&split_set, "OAP".to_string());
//    if post_request(&split_set) {
//        println!("Initiating call from OAP to TAP");
//        post_request((&split_set));
//    }
    response
}

///generate phone number using shamir secret
/// split_rtm : split rtm
/// split_ir : split ir
/// split_oap : split oap
/// returns decoded
#[get("/<split_rtm>/<split_ir>/<split_oap>")]
pub fn recover_secret(split_rtm: String, split_ir: String, split_oap: String) -> String {
    let dec = base64::decode(&split_rtm).unwrap();
    let dec2 = base64::decode(&split_ir).unwrap();
    let dec3 = base64::decode(&split_oap).unwrap();
//    println!("dec1 = {:?}", &dec);
//    println!("dec2 = {:?}", &dec2);
//    println!("dec3 = {:?}", &dec3);
    SecretData::recover_secret(3, vec![dec, dec2, dec3])
        .unwrap_or("Wrong Key used!..".to_string())
}

//fn get_request(uri : String) -> String {
//    let mut response = reqwest::get(&uri).expect("failed to send request");
//    println!("Response Status: {}", response.status());
//
//    let mut buf = String::new();
//    response.read_to_string(&mut buf).expect("failed to read response");
//    buf
//
//}

//fn post_request(split_set: &SplitSet) -> bool {
//    let client = reqwest::Client::new();
//    let res = json!( {
//            "share_ir": split_set.share_ir,
//            "share_oap": split_set.share_oap,
//            "share_rtm": split_set.share_rtm,
//            "share_tap": split_set.share_tap
//    });
//    let mut response = client.post("http://localhost:8000/oap")
//        .json(&res)
//        .send()
//        .expect("Failed to send request");
//    assert_eq!(response.status(), 200);
//
//    let mut buf = String::new();
//    response.read_to_string(&mut buf).expect("Failed to read response");
//    println!("{}", buf);
//
//    if response.status() == 200 {
//        true
//    } else {
//        false
//    }
//}

