#[macro_use]
extern crate serde_derive;

#[derive(Serialize,Deserialize,Debug,PartialEq)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Root {
    pub SearchParkInfoService: Struct_SearchParkInfoService,
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_SearchParkInfoService {
    pub list_total_count: usize,
    pub RESULT: Struct_RESULT,
    pub row: Vec<Struct_row>,
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_row {
    pub P_IDX: String,
    pub P_PARK: String,
    pub P_LIST_CONTENT: String,
    pub P_ADDR: String,
    pub P_ZONE: String,
    pub P_DIVISION: String,
    pub P_IMG: String,
    pub P_ADMINTEL: String,
    pub LONGITUDE: f64,
    pub LATITUDE: f64,
    pub G_LONGITUDE: f64,
    pub G_LATITUDE: f64,
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_RESULT {
    pub CODE: String,
    pub MESSAGE: String,
}



extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn test() {
    let filename = "tests/seoul_parks.json";
    let mut file = File::open(filename).expect("failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file");
    let decoded: Root = serde_json::from_str(&contents).expect("failed to decode");

    let encoded = serde_json::to_string(&decoded).expect("failed to encode");
    let decoded2: Root = serde_json::from_str(&encoded).expect("failed to decode");
    assert_eq!(decoded, decoded2);
}