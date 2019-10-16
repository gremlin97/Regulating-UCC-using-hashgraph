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
// extern crate recrypt;
extern crate crypto;
extern crate bytes;
extern crate ring;

use rocket_prometheus::PrometheusMetrics;
use crypto :: digest :: Digest;
use crypto :: sha2 :: Sha256;
use merkletree_rs::{db, MerkleTree, TestValue};
use std::sync::Mutex;
use ring::digest;
mod smt_number;
mod vid_generation;
mod tsp1;
mod client_call;
mod messagestatus;


pub  fn  hash_leaf (value: & [ u8 ]) -> [ u8 ; 32 ] {
    let  mut sha = Sha256 :: new ();
    let  mut result = [ 0 ; 32 ];
    sha. input ( & value);
    sha. result ( & mut result);
    println!("result: {:?}", &result);
    result
}

pub fn digest_adapter(algorithm: &'static digest::Algorithm) -> Box<Fn(&[u8])->Vec<u8>> {
    let closure = move |x:&[u8]|  digest::digest(algorithm, x).as_ref().to_vec();
    return Box::new(closure);
}

// lazy_static! {
//     static ref TREE : Mutex<SMT> =  Mutex::new(SMT::new(vec![0x42 as u8], digest_adapter(&digest::SHA256)));
// }


fn main() {
    println!("Starting filtering service..");
    let mut s = String::from("Hello");
    // let bytes = unsafe { s.as_bytes_mut()};
    // let phone_number_t : String = "9034218122".to_string();
    // let mut vec : Vec<u8> = Vec::with_capacity(32);
    // let target = phone_number_t.as_bytes().to_vec();
    // sparse merkle tree
    // let tree = Tree { smt_tree : SMT::new(vec![0x42 as u8], digest_adapter(&digest::SHA256))};
    // let smt = SMT::new(vec![0x42 as u8], digest_adapter(&digest::SHA256));
    // let mut key: Vec<Vec<u8>>= Vec::new();
    // key.push((*digest_adapter(&digest::SHA256))(&"abc".as_bytes().to_vec()));

    // let keys = Key::from_vec(key.clone());
    // let d = D::from_vec(key.clone());
    // let mut c = CacheNothing{};
    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount("/", routes![tsp1::initiate_call, tsp1::terminate_call, smt_number::add_customer, messagestatus::check_user_pref, messagestatus::generate_proof, 
        messagestatus::non_inclusion])
        .mount("/vid", routes![vid_generation::generate_splits, vid_generation::recover_secret])
        .mount("/add", routes![smt_number::add_customer])
        .launch();
}

pub fn smt_trial() {

}

#[get("/<number>/<name>")]
pub fn register_customer(number : String, name: String) -> String {

    let phone_number : String = number.to_string();
    let mut sto = db::Db::new("test".to_string(), true);
    let mut mt = MerkleTree::new(&mut sto, 140 as u32);
    let val: TestValue = TestValue {
        bytes: phone_number.as_bytes().to_vec(),
        index_length: 10,
    };
    mt.add(&val).unwrap();
    mt.print_full_tree();
    format!("Hi")

}

