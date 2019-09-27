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


use merkle_tree::{MerkleTree, SerializationFormat};
use crypto :: digest :: Digest;
use crypto :: sha2 :: Sha256;

mod smt_number;
mod vid_generation;
mod tsp1;
mod client_call;
mod messagestatus;




// lazy_static! {
//     static ref sto : Arc<Mutex<db::Db>> = db::Db::new("test".to_string(), true);
// //    pub static ref mt : MerkleTree<'static> = MerkleTree::new(140 as u32);
// //    static ref mt: Mutex<MerkleTree<'static>> = {
// //        let mut m = MerkleTree::new(&mut sto, 140 as u32);
// //        Mutex::new(m)
// //    };

// }


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
    let spar = "b".to_string().into_bytes();
    let serialized_hashed_a =
            hash_leaf ( &spar);
    
    // println!("{:?}", &spar);
     let mut merkle_tree = MerkleTree::from(&mut ["a", "b", "c"], SerializationFormat::Json);
    // вызываем построение дерева    
    merkle_tree.build().unwrap();
    // печатаем вычисленный хэш рут дерева
    println!("Merkle tree root hash: {:?}", merkle_tree.get_merkle_root());
    // печатаем пруф-путь для транзакции b
    println!("Merkle tree audit proot: {:?}", merkle_tree.audit_proof(&[172, 141, 131, 66, 187, 178, 54,
     45, 19, 240, 165, 89, 163, 98, 27, 180, 7, 1, 19, 104, 137, 81, 100, 182, 40, 165, 79, 127, 195, 63, 
     196, 60])
     .unwrap());
    // добавляем в дерево хэш транзакции d
    merkle_tree.push(&String::from("d"));
    // печатаем рут хэш дерева
    println!("Merkle tree root hash: {:?}", merkle_tree.get_merkle_root());
    // печатаем пруф-путь для транзакции b
    println!("Merkle tree audit proot: {:?}", merkle_tree.audit_proof(&[172, 141, 131, 66, 187, 178, 54, 
    45, 19, 240, 165, 89, 163, 98, 27, 180, 7, 1, 19, 104, 137, 81, 100, 182, 40, 165, 79, 127, 195, 63, 
    196, 60])
    .unwrap());
    
    rocket::ignite()
        // .manage(MySto{ sto : db::Db::new("test".to_string(), true)})
        .mount("/", routes![smt_number::register_customer, smt_number::set_reminders, tsp1::initiate_call, tsp1::terminate_call, smt_number::add_customer, messagestatus::check_user_pref])
        .mount("/vid", routes![vid_generation::generate_splits, vid_generation::recover_secret])
        .mount("/add", routes![smt_number::add_customer])
        .launch();
}



