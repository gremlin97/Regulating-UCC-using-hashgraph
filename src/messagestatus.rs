use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;

use merkletree_rs::{db, MerkleTree, TestValue, Value};
use serde::{Serialize, Deserialize}; // imports both the trait and the derive macro

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageBlock {
    message : String,
    user_number : String,
    content_provider_id : String,
    category : String,
    header_id : String,
    template_id : String,
    purpose : String,
    rtm_id : String,
}

//impl MessageBlock {
//    pub fn get_details(&self) -> String {
//        let mut result: String = self::MessageBlock.message;
//        result
//    }
//}

#[post("/user", format = "application/json", data = "<message_block>")]
pub fn check_user_pref(message_block : Json<MessageBlock>) -> String {
    println!("user {:?}", message_block);

    {
        let mut sto = db::Db::new("test".to_string(), true);
        let mut mt = MerkleTree::new(&mut sto, 140 as u32);

        let phone_number : String = "9034218120".to_string();
        let phone_number_t : String = "9034218122".to_string();
        let val: TestValue = TestValue {
            bytes: phone_number.as_bytes().to_vec(),
            index_length: 10,
        };

        let val2: TestValue = TestValue {
            bytes: phone_number_t.as_bytes().to_vec(),
            index_length: 10,
        };

        mt.add(&val).unwrap();
        let mp = mt.generate_proof(val.hi());
        println!("{:?}", mp);

        let mp2 = mt.generate_proof(val.hi());

        // check if the value exist
        let v =
            merkletree_rs::verify_proof(mt.get_root(), &mp, val.hi(), val.ht(), mt.get_num_levels());
        println!("{:?}", v);

        let v =
            merkletree_rs::verify_proof(mt.get_root(), &mp2, val2.hi(), val2.ht(), mt.get_num_levels());
        println!("{:?}", v);
    }

    //check if user has not blocked content provider
    //check if category matches user preferred
    let user_content_consent = user_consent::user_consent_contentprov(&message_block.user_number,
                                                        &message_block.content_provider_id);
    let user_pref_consent = user_consent::user_pref(&message_block.user_number,
                                      &message_block.category);


    if !user_content_consent && !user_pref_consent {
        format!("No consent or preference matched")
    } else{
        format!("consent and preference matched")
        /*store the transaction onto blockchain
        2. get registered content provider template contsraints
        3. match and proceed accordingly
        4. if all true, generate and assign a vId to number.
        5.?? no use of id on blockchain if it's changing or make a different service
        6. now as user consent and pref are checked, check template and content provider
        7. content provider can have many different templates
        */

//        let template_constraint = get_template_constraint(&message_block.user)



    }


}

//#[post("/register_user/", format="application/json", data="")]


mod user_consent {
    pub fn user_consent_contentprov(user_number : &String, cntProv_id : &String) -> bool {
        false
    }

    pub fn user_pref(user_number : &String, category : &String) -> bool {
        true
    }
}

