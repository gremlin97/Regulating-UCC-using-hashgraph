#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate merkletree_rs;
extern crate serde;
extern crate threshold_secret_sharing as tss;
#[macro_use]
extern crate rocket_contrib;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;
extern crate shamir;
extern crate base64;
extern crate recrypt;

use rocket_contrib::json::{Json};
use merkletree_rs::{db, MerkleTree, TestValue, Value};
use std::sync::Mutex;
use std::sync::Arc;

mod smt_number;
mod vid_generation;
mod tsp1;
mod client_call;




// lazy_static! {
//     static ref sto : Arc<Mutex<db::Db>> = db::Db::new("test".to_string(), true);
// //    pub static ref mt : MerkleTree<'static> = MerkleTree::new(140 as u32);
// //    static ref mt: Mutex<MerkleTree<'static>> = {
// //        let mut m = MerkleTree::new(&mut sto, 140 as u32);
// //        Mutex::new(m)
// //    };

// }

//struct Mt<'a> {
//    mt : &'a MerkleTree<'a>,
//}
pub struct MySto {
    sto : db::Db
}
//pub struct MyDb(db::Db);
pub struct MyTree<'a>(MerkleTree<'a>);

fn main() {
    println!("Starting filtering service..");
//    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

//    let mut sto = db::Db::new("test".to_string(), true);
// //    println!("sto {:?}", sto);
//    let mut mt = MerkleTree::new(&mut sto, 140 as u32);
// //
//    let phone_number : String = "9034218120".to_string();
//    let phone_number_t : String = "9034218122".to_string();
//    let val: TestValue = TestValue {
//        bytes: phone_number.as_bytes().to_vec(),
//        index_length: 10,
//    };
// //
// //    let val2: TestValue = TestValue {
// //        bytes: phone_number_t.as_bytes().to_vec(),
// //        index_length: 10,
// //    };
// //
//    mt.add(&val).unwrap();
//    let mp = mt.generate_proof(val.hi());
//    println!("{:?}", mp);
// //
// //    let mp2 = mt.generate_proof(val.hi());
// //
//    // check if the value exist
//    let v =
//        merkletree_rs::verify_proof(mt.get_root(), &mp, val.hi(), val.ht(), mt.get_num_levels());
//    println!("{:?}", v);
// //
// //    let v =
// //        merkletree_rs::verify_proof(mt.get_root(), &mp2, val2.hi(), val2.ht(), mt.get_num_levels());
// //    println!("{:?}", v);
// //
// //
// // check if the don't value exist (in that case, the 'ht' will be an empty value)
//    let v = merkletree_rs::verify_proof(
//        mt.get_root(),
//        &mp,
//        val.hi(),
//        merkletree_rs::constants::EMPTYNODEVALUE,
//        mt.get_num_levels(),
//    );
//    println!("{:?}", v);

//    set_reminder();
//    let config = MySto("user_registration".to_string());
//    let mut sto = (db::Db::new("test".to_string(), true));
//    let  mt = MyTree(MerkleTree::new(&mut sto, 140 as u32));


    rocket::ignite()
        // .manage(MySto{ sto : db::Db::new("test".to_string(), true)})
        .mount("/", routes![smt_number::register_customer, smt_number::set_reminders, tsp1::initiate_call, tsp1::terminate_call, smt_number::add_customer])
        .mount("/vid", routes![vid_generation::generate_splits, vid_generation::recover_secret])
        .mount("/add", routes![smt_number::add_customer])
        .launch();
}

//
#[get("/<details>")]
pub fn set_reminder(details : String) -> String {
   //service to database
//    println!("values key{} value {}", HASHMAP.get("amber").unwrap());
//    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
//    println!("values keyvalue {:?}", HOSTNAME);
//    format!("Task: {} saved for at time {}, sto {}.", details, mt.0, sto.0)
    let mut sto = db::Db::new("test".to_string(), true);
       let mut mt = MerkleTree::new(&mut sto, 140 as u32);
       let val: TestValue = TestValue {
       bytes: details.as_bytes().to_vec(),
       index_length: 10,
   };
   let mp = mt.generate_proof(val.hi());
   println!("{:?}", mp);
   // check if the value exist
   let v =
       merkletree_rs::verify_proof(mt.get_root(), &mp, val.hi(), val.ht(), mt.get_num_levels());
   println!("{:?}", v);
//
    "Sdfsd".to_string()


}



