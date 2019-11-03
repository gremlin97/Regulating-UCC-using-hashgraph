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
mod smt_number;
mod vid_generation;
mod tsp1;
mod client_call;
mod messagestatus;
mod rtm;
mod ir;


fn main() {
    println!("Starting filtering service..");
    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount("/", routes![tsp1::initiate_call, tsp1::terminate_call, smt_number::add_customer, messagestatus::add_user_pref, messagestatus::generate_proof, 
        messagestatus::non_inclusion])
        .mount("/vid", routes![vid_generation::generate_splits, vid_generation::recover_secret])
        .mount("/add", routes![smt_number::add_customer])
        .mount("/", routes![rtm::trigger_ad_service, ir::trigger_ir_service])
        .launch();
}
