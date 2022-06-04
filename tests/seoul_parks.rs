#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Root {
    pub SearchParkInfoService: Struct_SearchParkInfoService,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_SearchParkInfoService {
    pub list_total_count: usize,
    pub RESULT: Struct_RESULT,
    pub row: Vec<Struct_row>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
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

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_RESULT {
    pub CODE: String,
    pub MESSAGE: String,
}


fn test_runner<T>(s: &str)
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a> + std::cmp::PartialEq,
{
    let decoded0: T = serde_json::from_str(s).expect("failed to decode");
    let encoded0 = serde_json::to_string(&decoded0).expect("failed to encode");
    let decoded1: T = serde_json::from_str(&encoded0).expect("failed to decode");
    let encoded1 = serde_json::to_string(&decoded1).expect("failed to encode");

    assert_eq!(encoded0, encoded1);
    assert!(std::cmp::PartialEq::eq(&decoded0, &decoded1));
}

#[test]
fn test() {
    let filename = "tests/seoul_parks.json";
    let contents = std::fs::read_to_string(filename).expect("failed to read");
    test_runner::<Root>(&contents);
}