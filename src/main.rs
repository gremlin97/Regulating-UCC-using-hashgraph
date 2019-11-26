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
extern crate merkle;
extern crate ring;

use ring::digest::{Algorithm, Context, SHA512};

use merkle::{Hashable, MerkleTree, Proof};



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

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

fn main() {
    let values = vec!["one", "two", "three", "four"];   
    let count = values.len();
    let tree = MerkleTree::from_vec(digest, values);

    assert_eq!(tree.count(), count, "ok");

    

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
