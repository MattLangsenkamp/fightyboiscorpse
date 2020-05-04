#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;
use rocket::State;
use std::sync::{Mutex};
use serde_json::json;

extern crate postgres;
//use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Claims  {
    claim_list:Mutex<Vec<Claim>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Claim {
    first_name:String,
    last_name:String,
    email:String,
    spot:u32
}

impl Claim {
    fn build_claim(jclaim: Json<Claim>) -> Claim {
        Claim {
            first_name: jclaim.first_name.clone(),
            last_name: jclaim.last_name.clone(),
            email: jclaim.email.clone(),
            spot: jclaim.spot.clone()
        }
    }

    fn claim_to_string(claim:Claim) -> String {
        let l = serde_json::to_string(&claim);
        return l.unwrap();
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/claim", data = "<jclaim>")]
fn claim_spot(jclaim:Json<Claim>,  claim_mutex: State<Claims>) -> &'static str {

    println!("{:?}", jclaim);
    let claims: &Claims = claim_mutex.inner();
    claims.claim_list.lock().unwrap().push(Claim::build_claim(jclaim));
    //let database_url = env::var("DATABASE_URL")
        //.expect("DATABASE_URL must be set");
    "Thank you for claiming a spot!"
}

#[get("/claim")]
fn get_claims(claim_mutex: State<Claims>) -> String {

    let claims: &Claims = claim_mutex.inner();
    let claim_list :&Vec<Claim> = &*claims.claim_list.lock().unwrap();
    return json!(claim_list.clone()).to_string();
}

fn main() {
    rocket::ignite()
        .manage(Claims {
            claim_list:Mutex::new(Vec::new())
        })
        .mount("/", routes![index, claim_spot, get_claims])
        .launch();
}

