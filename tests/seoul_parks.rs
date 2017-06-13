#[macro_use]
extern crate serde_derive;

#[derive(Serialize,Deserialize)]
pub struct Root {
    pub SearchParkInfoService: Struct_SearchParkInfoService,
}

#[derive(Serialize,Deserialize)]
pub struct Struct_SearchParkInfoService {
    pub list_total_count: usize,
    pub RESULT: Struct_RESULT,
    pub row: Vec<Struct_row>,
}

#[derive(Serialize,Deserialize)]
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

#[derive(Serialize,Deserialize)]
pub struct Struct_RESULT {
    pub CODE: String,
    pub MESSAGE: String,
}

