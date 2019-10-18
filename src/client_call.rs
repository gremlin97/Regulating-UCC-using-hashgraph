use reqwest;
use serde::{Deserialize, Serialize};
use reqwest::Response;
use std::io::{Read};


///MessageBlock: block containing information from RTM to IR
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageBlock {
    pub message : String,
    pub user_number : String,
    pub content_provider_id : String,
    pub category : String,
    pub header_id : String,
    pub template_id : String,
    pub purpose : String,
    pub rtm_id : String,
}

impl MessageBlock {
   pub fn get_string(self: &Self) -> String {
       let message_block_object = json!({
           "message" : self.message,
           "user_number" : self.user_number,
           "content_provider_id": self.content_provider_id,
           "category": self.category,
           "header_id": self.header_id,
           "template_id": self.template_id,
           "purpose": self.purpose,
           "rtm_id": self.rtm_id
       });
       message_block_object.to_string()

   }
}

//SplitSet: splits the number into shares
#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SplitSet {
    pub share_rtm: String,
    pub share_oap: String,
    pub share_ir: String,
    pub share_tap : String
}

pub fn get_request(uri : String) -> Response {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8000/vid/9034218120/4")
    .send()
    .expect("Failed to send request");
    response
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

