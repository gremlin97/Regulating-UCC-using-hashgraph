///receives ecrypted keys from rtm
///use tsp private key to decrypt the share
/// sends atleast 2/3
///better then only private-public as then TSP can have access to data anytime.
/// send call to tap after that

use rocket_contrib::json::{Json};
use shamir::SecretData;
use rocket::response::status;
use std;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct SplitSet {
    pub share_rtm: String,
    pub share_oap: String,
    pub share_ir: String,
    pub share_tap : String
}

/// post call as it will be not be accessible from browser safety
#[post("/oap", format = "application/json", data = "<split_set>")]
pub fn initiate_call(split_set : Json<SplitSet>) -> String {
//    println!("Received from RTM, split_set: {:?}.", split_set);

    ///decrypt share_oap
    /// call shamir to decrypt the number.
    /// no need to call ir again
    /// OR call ir to decrypt share_ir

    let key_share_oap = base64::decode(&split_set.share_oap).unwrap();
    let key_share_ir = base64::decode(&split_set.share_ir).unwrap();
    let key_share_rtm = base64::decode(&split_set.share_rtm).unwrap();
    let retrieved_user_number = SecretData::recover_secret(3, vec![key_share_oap, key_share_ir,key_share_rtm])
                                            .unwrap_or("Wrong Key used!..".to_string());

    println!("retrieved user number at OAP..: {}", retrieved_user_number);
    assert_eq!(retrieved_user_number, "9034218120");
    let split_set = crate::client_call::SplitSet{
        share_rtm: split_set.share_rtm.to_string(),
        share_oap: split_set.share_oap.to_string(),
        share_ir: split_set.share_ir.to_string(),
        share_tap: split_set.share_tap.to_string()
    };
    if retrieved_user_number == "9034218120" {
        println!("Initiating call from OAP to TAP");
        crate::client_call::post_request(&split_set, "TAP".to_string());
        "Accepted".to_string()
    } else {
        "Not Accepted".to_string()
    }
}

#[post("/tap", format = "application/json", data = "<split_set>")]
pub fn terminate_call(split_set : Json<SplitSet>) -> String {
//    println!("Received from OAP, split_set: {:?}.", split_set);

    ///decrypt share_oap
    /// call shamir to decrypt the number.
    /// no need to call ir again
    /// OR call ir to decrypt share_ir

    let key_share_tap = base64::decode(&split_set.share_tap).unwrap();
    let key_share_ir = base64::decode(&split_set.share_ir).unwrap();
    let key_share_rtm = base64::decode(&split_set.share_rtm).unwrap();
    let retrieved_user_number = SecretData::recover_secret(3, vec![key_share_tap, key_share_ir,key_share_rtm])
        .unwrap_or("Wrong Key used!..".to_string());

    println!("retrieved user number at TAP..: {}", retrieved_user_number);
    assert_eq!(retrieved_user_number, "9034218120");
    if retrieved_user_number == "9034218120" {
        "Accepted".to_string()
    } else {
        "Not Accepted".to_string()
    }
}


