/// Interact with hedera
/// will act as middleware between rtm , oap, tap and hedera
use rocket_contrib::json::{Json, JsonValue};
use merkletree_rs::{db, MerkleTree, TestValue, Value};
use reqwest;
use reqwest::Response;
use std::io::{Read};
use serde::{Deserialize, Serialize};
use crate::client_call::SplitSet;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdContentIr {
    pub mode : String,
    pub category : String,
    pub time : String,
    pub day : String,
}

//check user preference
//generate vid
//send splits back
#[post("/ir/filter_users/<user_id>", format = "application/json", data = "<ad_target_ir>")]
pub fn trigger_ir_service( ad_target_ir : Json<AdContentIr>, user_id : String) -> Json<SplitSet>{
    let ad_target_ir : AdContentIr = ad_target_ir.into_inner();
    println!("{:?}", ad_target_ir);


    let uri = "http://localhost:8000/proof/".to_string() + &user_id;
    let client = reqwest::Client::new();
    let mut response = client.post(&uri)
        .json(&ad_target_ir)
        .send()
        .expect("Failed to send request");
    assert_eq!(response.status(), 200);

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);

    if buf == "true" {
        println!("{} has validated the coresspoding preference.", user_id);
        let client = reqwest::Client::new();
        let uri = "http://localhost:8000/vid/".to_string() + &user_id + "/3";
        let mut response = client.get(&uri)
        .send()
        .expect("Failed to send request");

        let mut buf = String::new();
        if let Ok(splitset) = response.json::<SplitSet>() {
            // println!("{:?}", splitset);
            Json(splitset)
        }
        else {
            let split_set = SplitSet{
                share_rtm: "null".to_string(),
                share_oap: "null".to_string(),
                share_ir: "null".to_string(),
                share_tap : "null".to_string(),
            };
        Json(split_set)
        }
    }
    else {
        println!("{} has not validated the coresspoding preference", user_id);
        println!("{} with message is stopped at RTM due to preference.", user_id);
        let split_set = SplitSet{
        share_rtm: "null".to_string(),
        share_oap: "null".to_string(),
        share_ir: "null".to_string(),
        share_tap : "null".to_string(),
        };
        Json(split_set)
    }
}

 