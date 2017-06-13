#[macro_use]
extern crate serde_derive;

#[derive(Serialize,Deserialize)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Root {
    pub batchcomplete: String,
    #[serde(rename = "continue")]
    pub field_continue: Struct_continue,
    pub query: Struct_query,
}

#[derive(Serialize,Deserialize)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_query {
    pub recentchanges: Vec<Struct_recentchanges>,
}

#[derive(Serialize,Deserialize)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_recentchanges {
    #[serde(rename = "type")]
    pub field_type: String,
    pub ns: usize,
    pub title: String,
    pub pageid: usize,
    pub revid: usize,
    pub old_revid: usize,
    pub rcid: usize,
    pub user: String,
    pub oldlen: usize,
    pub newlen: usize,
    pub anon: Option<String>,
    pub bot: Option<String>,
    pub minor: Option<String>,
    pub new: Option<String>,
}

#[derive(Serialize,Deserialize)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_continue {
    pub rccontinue: String,
    #[serde(rename = "continue")]
    pub field_continue: String,
}



extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn test() {
    let filename = "tests/wikipedia_changes.json";
    let mut file = File::open(filename).expect("failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file");
    let _: Root = serde_json::from_str(&contents).expect("failed to decode");
}