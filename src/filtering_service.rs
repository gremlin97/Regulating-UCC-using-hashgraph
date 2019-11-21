use rocket_contrib::json::{Json, JsonValue};
use merkletree_rs::{db, MerkleTree, TestValue, Value};
use crate::client_call::{SplitSet, MessageBlock};
use reqwest;
use reqwest::Response;
use std::io::{Read};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageBody {
    pub Message : String, 
	pub Number : String, 
    pub ContentProviderId : String,
    pub Category : String,
	pub HeaderId : String,
	pub TemplateId : String,
	pub Purpose : String,
	pub RtmId : String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdContent {
    pub mode : String,
    pub category : String,
    pub time : String,
    pub day : String,
}

#[post("/filtering_service/send_message/<user_id>", format = "application/json", data = "<message_body>")]
pub fn send_message_after_filtering( message_body : Json<MessageBody>, user_id : String) {
    // call for consent
    let consent_contentprovider_by_customer = true;
    // call for preference 
    // preference being checked at RTM for better flow and score usage.
    if !consent_contentprovider_by_customer {
        println!("No Consent or preference for the said content provider by the user.")
    } else {
        //check if template and header matches
        //generate vid
        //route message to selected RTM
        let message_body : MessageBody = message_body.into_inner();
        let ad_target = AdContent {
                mode : message_body.Purpose,
                category : message_body.Category,
                time : message_body.Message,
                day : "monday".to_string()
            };
        let uri = "http://localhost:8000/rtm/send_ads/".to_string() + &user_id;
        let client = reqwest::Client::new();
        let mut response = client.post(&uri)
        .json(&ad_target)
        .send()
        .expect("Failed to send request");
    assert_eq!(response.status(), 200);

    }
}


