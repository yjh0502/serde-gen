#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Root {
    pub batchcomplete: String,
    #[serde(rename = "continue")]
    pub field_continue: Struct_continue,
    pub query: Struct_query,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_query {
    pub recentchanges: Vec<Struct_recentchanges>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
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

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Struct_continue {
    pub rccontinue: String,
    #[serde(rename = "continue")]
    pub field_continue: String,
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
    let filename = "tests/wikipedia_changes.json";
    let contents = std::fs::read_to_string(filename).expect("failed to read");
    test_runner::<Root>(&contents);
}