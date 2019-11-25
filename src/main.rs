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
extern crate shamir;
extern crate base64;
// extern crate recrypt;
extern crate crypto;
extern crate bytes;

use rocket_prometheus::PrometheusMetrics;
use rust_sodium::crypto::secretbox;
mod smt_number;
mod vid_generation;
mod tsp1;
mod client_call;
mod preference_service;
mod rtm;
mod ir;
mod filtering_service;
fn main() {

//     let mut merk = Merk::open("./merk.db").unwrap();
//     let batch_size = 20;
//     // let batch = make_batch_seq(0..batch_size);
// //     let batch = [
// //     (b"key", Op::Put(b"value")),
// //     (b"key2", Op::Put(b"value2")),
// //     (b"key3", Op::Put(b"value3")),
// //     (b"key4", Op::Delete)
// // ];
// let batch = &[
//          (vec![1, 2, 3], Op::Put(vec![4, 5, 6])), // puts value [4,5,6] to key [1,2,3]
//          (vec![4, 5, 6], Op::Delete) // deletes key [4,5,6]
//      ];
// merk.apply(&batch).expect("apply failed");
//    let key = secretbox::gen_key();
// let nonce = secretbox::gen_nonce();
// let plaintext = b"some data";
// let ciphertext = secretbox::seal(plaintext, &nonce, &key);
// let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();
// assert!(plaintext == &their_plaintext[..]);


    println!("Starting filtering service..");
    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount("/", routes![tsp1::initiate_call, tsp1::terminate_call, smt_number::add_customer, preference_service::add_user_pref, preference_service::generate_proof, 
        preference_service::non_inclusion])
        .mount("/vid", routes![vid_generation::generate_splits, vid_generation::recover_secret])
        .mount("/add", routes![smt_number::add_customer])
        .mount("/", routes![rtm::trigger_ad_service, ir::trigger_ir_service, filtering_service::send_message_after_filtering])
        .launch();
}
