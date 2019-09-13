use rocket_contrib::json::{Json, JsonValue};
use merkletree_rs::{db, MerkleTree, TestValue, Value};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::OnceCell;
use rocket::State;


static HOSTNAME : OnceCell<String> = OnceCell::INIT;
//#[derive(Clone)]
//struct Node<'a> {
//    mt : &'a MerkleTree
//}
//lazy_static! {
//
////      static ref mt: Mutex<MerkleTree<'static>> = {
////      let mut m = MerkleTree::new(&mut sto, 140 as u32);
////      Mutex::new(m)
////    };
////    static ref mt : MerkleTree =
//    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new("ff"));
//}


#[get("/<details>")]
pub fn set_reminders(details : String) -> String {
    //service to database
//    println!("values key{} value {}", HASHMAP.get("amber").unwrap());
//    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
        println!("values keyvalue {:?}", HOSTNAME);


    format!("Task: {} saved for at time {}.", details, details)

}

#[get("/<number>/<name>")]
pub fn register_customer(number : String, name : String) -> String {

    let phone_number : String = number.to_string();
    let mut sto = db::Db::new(phone_number.to_string(), true);
    let mut mt = MerkleTree::new(&mut sto, 140 as u32);
    let mut val: TestValue = TestValue {
        bytes: phone_number.as_bytes().to_vec(),
        index_length: 10,
    };
    mt.add(&val).unwrap();
//    let mp = mt.generate_proof(val.hi());
////    println!("{:?}", mp);
////
////
////    // check if the value exist
////    let v =
////        merkletree_rs::verify_proof(mt.get_root(), &mp, val.hi(), val.ht(), mt.get_num_levels());
////    println!("{:?}", v);
//
//    let t = generate_proof(&val, &mut mt);
//    let v = is_exists(&mut mt, &mut val);

//    format!("Hi {}. We have registered you {}, {}!", number, t, v)

        format!("Hi")

}

fn generate_proof(val : &TestValue, mt: &mut MerkleTree ) -> bool {
    let phone_number : String = "cs".to_string();
    let mut val: TestValue = TestValue {
        bytes: phone_number.as_bytes().to_vec(),
        index_length: 10,
    };
    let mp = mt.generate_proof(val.hi());
    print!("proof {:?}", mp);
    true
}

fn is_exists(mt : &mut MerkleTree, val : &TestValue) -> bool {
    let mp = mt.generate_proof(val.hi());
    let v = merkletree_rs::verify_proof(mt.get_root(), &mp,
                                        val.hi(), val.ht(),
                                        mt.get_num_levels());
//    println!("{:?}", v);
//    v
    true
}

