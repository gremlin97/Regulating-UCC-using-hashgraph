use rocket_contrib::json::{Json, JsonValue};
use merkletree_rs::{db, MerkleTree, TestValue, Value};
use crate::client_call::{SplitSet, MessageBlock};


/// User pref are stored in a sparse-merkle tree
/// initiate a db connection 
///check if user has not blocked content provider
///check if category matches user preferrence
/*store the transaction onto blockchain
2. get registered content provider template contsraints
3. match and proceed accordingly
4. if all true, generate and assign a vId to number.
5.?? no use of id on blockchain if it's changing or make a different service
6. now as user consent and pref are checked, check template and content provider
7. content provider can have many different templates
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreference {
    pub mode : String,
    pub category : String,
    pub time : String,
    pub day : String,
}

impl UserPreference {
   pub fn get_string(self: &Self) -> String {
       let message_block_object = json!({
           "a_mode" : self.mode,
           "b_category" : self.category,
           "preference" : 1
        });
       message_block_object.to_string()

   }
}

#[post("/user/<user_id>", format = "application/json", data = "<user_preference>")]
pub fn add_user_pref(user_preference : Json<UserPreference>, user_id : String) -> String {
    let user_preference : UserPreference = user_preference.into_inner();
    
        let mut sto = db::Db::new(user_id.to_string(), false);
        let mut mt = MerkleTree::new(&mut sto, 140 as u32);
        
        let user_preference_rtm : String = user_preference.get_string();
        println!("user_preference_rtm {}, length {}", &user_preference_rtm, &user_preference_rtm.len());

        let val: TestValue = TestValue {
            bytes: user_preference_rtm.as_bytes().to_vec(),
            index_length: 30,
        };
        mt.add(&val).unwrap();
        println!("{:?}", mt.get_root());
        format!("completed")
    
}

#[post("/proof/<user_id>", format = "application/json", data = "<user_preference>")]
pub fn generate_proof(user_preference : Json<UserPreference>, user_id : String) -> String {
    let mut sto = db::Db::new(user_id.to_string(), false);
    let mut mt = MerkleTree::new(&mut sto, 140);
    let user_preference : UserPreference = user_preference.into_inner();

   let user_preference_rtm : String = user_preference.get_string();
   println!("user_preference_rtm {} length {}", &user_preference_rtm, &user_preference_rtm[0..30]);
        let val: TestValue = TestValue {
            bytes: user_preference_rtm.as_bytes().to_vec(),
            index_length: 30,
    };
    println!("I am here! {:?}", mt.get_root());
    let mp = mt.generate_proof(val.hi());
    let v = merkletree_rs::verify_proof(
                mt.get_root(), 
                &mp, 
                val.hi(),
                val.ht(),
                mt.get_num_levels());
    println!("OKAY {:?}", v);
    format!("{}", v)
}


#[post("/non_inclusion/<user_id>", format = "application/json", data = "<user_preference>")]
pub fn non_inclusion(user_preference : Json<UserPreference>, user_id : String) {
    let mut sto = db::Db::new(user_id.to_string(), false);
    let mut mt = MerkleTree::new(&mut sto, 140);
        let user_preference : UserPreference = user_preference.into_inner();

    let user_preference_rtm : String = user_preference.get_string();
        let val: TestValue = TestValue {
            bytes: user_preference_rtm.as_bytes().to_vec(),
            index_length: 30,
    };
    let mp = mt.generate_proof(val.hi());

    // check if the don't value exist (in that case, the 'ht' will be an empty value)
    let v = merkletree_rs::verify_proof(
        mt.get_root(),
        &mp,
        val.hi(),
        merkletree_rs::constants::EMPTYNODEVALUE,
        mt.get_num_levels(),
    );
    println!("{:?}", v);
}


fn get_splits_number() {
    let url = "http://localhost:8000/vid/9034218120/4";
    let mut response = crate::client_call::get_request(url.to_string());
    if let Ok(split_set) = response.json::<SplitSet>() {
         println!("Forwarding the keys to OAP..");
         crate::client_call::post_request(&split_set, "OAP".to_string(), "What are you doing ananya?".to_string());
    }
}


