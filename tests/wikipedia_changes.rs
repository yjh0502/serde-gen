pub struct Root {
    pub batchcomplete: String,
    pub field_continue: Struct_continue,
    pub query: Struct_query,
}

pub struct Struct_query {
    pub recentchanges: Vec<Struct_recentchanges>,
}

pub struct Struct_recentchanges {
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

pub struct Struct_continue {
    pub rccontinue: String,
    pub field_continue: String,
}
