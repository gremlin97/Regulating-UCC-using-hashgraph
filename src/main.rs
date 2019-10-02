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
extern crate lazy_static;
extern crate shamir;
extern crate base64;
extern crate recrypt;
extern crate merkle_tree;
extern crate crypto;
extern crate bytes;
use rocket_prometheus::PrometheusMetrics;

use crypto :: digest :: Digest;
use crypto :: sha2 :: Sha256;
use merkletree_rs::{db, MerkleTree, TestValue};
use std::sync::Mutex;



mod smt_number;
mod vid_generation;
mod tsp1;
mod client_call;
mod messagestatus;



struct HitCount {
    sto : Mutex<db::Db>,
}


pub  fn  hash_leaf (value: & [ u8 ]) -> [ u8 ; 32 ] {
    let  mut sha = Sha256 :: new ();
    let  mut result = [ 0 ; 32 ];
    sha. input ( & value);
    sha. result ( & mut result);
    println!("result: {:?}", &result);
    result
}

fn main() {
    println!("Starting filtering service..");
    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount("/", routes![register_customer,  tsp1::initiate_call, tsp1::terminate_call, smt_number::add_customer, messagestatus::check_user_pref])
        .mount("/vid", routes![vid_generation::generate_splits, vid_generation::recover_secret])
        .mount("/add", routes![smt_number::add_customer])
        .launch();
}

#[get("/<number>/<name>")]
pub fn register_customer(number : String, name: String) -> String {

    let phone_number : String = number.to_string();
    let mut sto = db::Db::new("test".to_string(), true);
    let mut mt = MerkleTree::new(&mut sto, 140 as u32);
    let mut val: TestValue = TestValue {
        bytes: phone_number.as_bytes().to_vec(),
        index_length: 10,
    };
    mt.add(&val).unwrap();
    mt.print_full_tree();
    format!("Hi")

}

