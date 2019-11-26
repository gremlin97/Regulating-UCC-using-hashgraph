///receives ecrypted keys from rtm
///use tsp private key to decrypt the share
/// sends atleast 2/3
///better then only private-public as then TSP can have access to data anytime.
/// send call to tap after that

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use rocket_contrib::json::{Json};
use shamir::SecretData;
use crate::client_call::SplitSet;
use std::fs::OpenOptions;
use std::io::prelude::*;


/// post call as it will be not be accessible from browser safety
///decrypt share_oap
/// call shamir to decrypt the number.
/// no need to call ir again
/// OR call ir to decrypt share_ir
#[post("/oap/<message>", format = "application/json", data = "<split_set>")]
pub fn initiate_call(split_set : Json<SplitSet>, message : String ) -> String {
    println!("Received from RTM, split_set: {:?} and message {}.", split_set, message);
    let key_share_oap = base64::decode(&split_set.share_oap).unwrap();
    let key_share_ir = base64::decode(&split_set.share_ir).unwrap();
    let key_share_rtm = base64::decode(&split_set.share_rtm).unwrap();
    let retrieved_user_number = SecretData::recover_secret(3, vec![key_share_oap, key_share_ir,key_share_rtm])
                                            .unwrap_or("Wrong Key used!..".to_string());

    println!("retrieved user number at OAP..: {}", &retrieved_user_number);
    //log to hedera about this interaction
    //move it later to ir
    let mut file = OpenOptions::new().append(true)
                                .open("OAP_LOGS.txt")
                                .expect("cannot open file");

    // let sys_time = SystemTime::now();
    // file.write_all(sys_time).expect("write failed");
    // let system_time = SystemTime::now();
    // let datetime: DateTime<Utc> = system_time.into();
    // println!("{}", datetime.format("%d/%m/%Y %T"));
    // file.write_all(datetime).expect("write failed");
    let log = "User Number: ".to_string() + &retrieved_user_number + " was accessed at OAP\n";
    file.write_all(log.as_bytes()).expect("write failed");
    println!("Hedera-hashgraph updated file append success\n");

    // assert_eq!(retrieved_user_number, "9034218124");
    let split_set = crate::client_call::SplitSet{
        share_rtm: split_set.share_rtm.to_string(),
        share_oap: split_set.share_oap.to_string(),
        share_ir: split_set.share_ir.to_string(),
        share_tap: split_set.share_tap.to_string()
    };
    if retrieved_user_number.len() == 10 {
        println!("Initiating call from OAP to TAP\n");
        crate::client_call::post_request(&split_set, "TAP".to_string(), message);
        "Accepted".to_string()
    } else {
        "Not Accepted".to_string()
    }
}

///decrypt share_oap
/// call shamir to decrypt the number.
/// no need to call ir again
/// OR call ir to decrypt share_ir
#[post("/tap/<message>", format = "application/json", data = "<split_set>")]
pub fn terminate_call(split_set : Json<SplitSet>, message : String){
//   println!("Received from OAP, split_set: {:?}.", split_set);
    let key_share_tap = base64::decode(&split_set.share_tap).unwrap();
    let key_share_ir = base64::decode(&split_set.share_ir).unwrap();
    let key_share_rtm = base64::decode(&split_set.share_rtm).unwrap();
    let retrieved_user_number = SecretData::recover_secret(3, vec![key_share_tap, key_share_ir,key_share_rtm])
        .unwrap_or("Wrong Key used!..".to_string());
    println!("retrieved user number at TAP..: {} and message {}\n", retrieved_user_number, message);
    //log onto hedera 
    let mut file = OpenOptions::new().append(true)
                                .open("TAP_LOGS.txt")
                                .expect("cannot open file");
    let log = "User Number: ".to_string() + &retrieved_user_number + " was accessed at TAP\n";
    file.write_all(log.as_bytes()).expect("write failed");
    println!("Hedera-hashgraph updated file append success");
}

