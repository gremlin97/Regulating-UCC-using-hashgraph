use reqwest;

//move client up lazy_static

pub struct UserPreference {
    pub mode : String,
    pub category : String,
    pub time : String,
    pub day : String,
}
pub fn register_preference(user_pref : &UserPreference) {
    let json_user_pref = json!( {
        "mode" : user_pref.mode,
        "category" : user_pref.category,
        "day" : user_pref.day,
        "time" : user_pref.time
    });
    let uri = "http://localhost:8002/registerPreference"
    let client = reqwest::Client::new();
    let mut response = client.post(uri)
        .json(&json_user_pref)
        .send()
        .expect("Failed to send request");

    //returns hash of updated tree
    let mut buf = String::new();
    respoese.read_to_string(&mut buf).expect("failed to read response");
    println!("{}", buf);
    //save blockchain
}

pub fn get_preference_root() {
    let client = reqwest::Client::new();
    let response = client.get("/getPreferenceRoot")
        .send()
        .expect("Failed to send request");
        println!("{}", response)
}

pub fn get_inclusion_proof(number : String, preference : String) {
    let client = reqwest::Client::new();
    let proof_parameters = json!( {
        "number" : number,
        "pref" : preference,
    });
    let mut response = client.post(uri)
        .json(&proof_parameters)
        .send()
        .expect("Failed to send request");
}

// fn post_call(uri : String) {
//     let mut response = client.post(uri)
//         .json(&proof_parameters)
//         .send()
//         .expect("Failed to send request");

//     let mut buf = String::new();
//     response.read_to_string(&mut buf).expect("Failed to read response");
//     println!("{}", buf);
// }



