use rocket_contrib::json::{Json, JsonValue};
use merkletree_rs::{db, MerkleTree, TestValue, Value};
use crate::client_call::{SplitSet, MessageBlock};
use reqwest;
use reqwest::Response;
use std::io::{Read};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdContent {
    pub mode : String,
    pub category : String,
    pub time : String,
    pub day : String,
}
#[post("/rtm/send_ads/<user_id>", format = "application/json", data = "<ad_target>")]
pub fn trigger_ad_service( ad_target : Json<AdContent>, user_id : String) -> String {
    let ad_target : AdContent = ad_target.into_inner();
    println!("{:?}", ad_target);

    //call filtering service to get the validated list
    let uri = "http://localhost:8000/ir/filter_users/".to_string() + &user_id;
    let client = reqwest::Client::new();
    let mut response = client.post(&uri)
        .json(&ad_target)
        .send()
        .expect("Failed to send request");
    assert_eq!(response.status(), 200);
    
    // if let Ok(split_set) = response.json::<SplitSet>() {
    //     if split_set.share_rtm == "null" {
    //         println!("Stopped ");
            
    //     }
    // }

    // let mut buf = String::new();
    // println!("Eligible numbers shamir-share received: {}", buf);
    println!("Transfer the ads and splits to OAP....");
    if let Ok(split_set) = response.json::<SplitSet>() {
         println!("Forwarding the keys to OAP..");
         crate::client_call::post_request(&split_set, "OAP".to_string(), "What are you doing Ananya?".to_string());
    }

    "------------------------- Transmission done succesfully!-------------------------".to_string()
    
}
