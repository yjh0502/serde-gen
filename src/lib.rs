use std::collections::HashMap;
use thiserror::Error;

mod ty;
pub use ty::Ty;

const RESERVED: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "Self", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "abstract", "alignof", "become", "box", "do", "final", "macro", "offsetof",
    "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized", "virtual", "yield",
];

fn field_name(name: &str) -> String {
    if RESERVED.contains(&name) {
        return format!("field_{}", name);
    }
    let replaced = name.replace(|c: char| !c.is_ascii_alphanumeric(), "_");

    let c = replaced.chars().next().unwrap();
    if c.is_ascii_digit() {
        return format!("field_{}", replaced);
    } else {
        replaced
    }
}

pub struct TyBuilder {
    names: HashMap<String, usize>,
    queue: Vec<(String, Vec<(String, Ty)>)>,
    any_ty: String,

    type_cache: HashMap<Ty, String>,
}

impl TyBuilder {
    pub fn new() -> Self {
        TyBuilder {
            names: HashMap::new(),
            queue: Vec::new(),
            any_ty: "::serde_json::Value".to_owned(),

            type_cache: HashMap::new(),
        }
    }

    pub fn set_any_ty(&mut self, any_ty: &str) {
        self.any_ty = any_ty.to_owned();
    }

    fn struct_name(&mut self, ident: String) -> String {
        let idx = self.names.entry(ident.clone()).or_insert(0);
        let name = if *idx == 0 {
            format!("Struct_{}", ident)
        } else {
            format!("Struct_{}_{}", ident, idx)
        };
        *idx = *idx + 1;
        name
    }

    fn ty_str(&mut self, key: &str, ty: Ty) -> String {
        if let Some(name) = self.type_cache.get(&ty) {
            return name.to_owned();
        }

        match ty {
            Ty::Bool => "bool".into(),
            Ty::I => "isize".into(),
            Ty::U => "usize".into(),
            Ty::F => "f64".into(),
            Ty::Char => "char".into(),
            Ty::Str(_) => "String".into(),
            Ty::Bytes => "Box<[u8]>".into(),

            Ty::Some(ty) => format!("Option<{}>", self.ty_str(key, *ty)),

            Ty::Seq(ty) => format!("Vec<{}>", self.ty_str(key, *ty)),
            Ty::Map(m) => {
                let name = self.struct_name(key.to_owned());
                self.type_cache.insert(Ty::Map(m.clone()), name.clone());
                self.queue.push((name.clone(), m));
                name
            }

            Ty::None => {
                // none only, no detailed type info
                "Option<()>".to_owned()
            }

            // Any, Unit, Some,
            _ => self.any_ty.clone(),
        }
    }

    pub fn derive_headers() -> &'static str {
        r#"#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]"#
    }

    pub fn build(&mut self, root_name: &str, mut ty: Ty) -> String {
        let mut s = String::new();
        let mut root_name = root_name.to_owned();

        if let Ty::Seq(v) = ty {
            root_name = format!("{}Elem", root_name);
            ty = v.as_ref().clone();
        }

        if let Ty::Map(v) = ty {
            self.queue.push((root_name, v));
        }

        while let Some((name, def)) = self.queue.pop() {
            s.push_str(&format!(
                r#"{}
pub struct {} {{
"#,
                Self::derive_headers(),
                name
            ));

            for (name, ty) in def.into_iter() {
                let field_name = field_name(&name);
                let prefix = if field_name == name {
                    "".to_owned()
                } else {
                    format!("    #[serde(rename = \"{}\")]\n", name)
                };
                s.push_str(&format!(
                    "{}    pub {}: {},\n",
                    prefix,
                    field_name,
                    self.ty_str(&name, ty)
                ));
            }
            s.push_str("}\n\n");
        }
        s
    }

    pub fn build_test_runner(name: &str) -> String {
        return format!(
            r#"
fn {}<T>(s: &str)
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a> + std::cmp::PartialEq,
{{
    let decoded0: T = serde_json::from_str(s).expect("failed to decode");
    let encoded0 = serde_json::to_string(&decoded0).expect("failed to encode");
    let decoded1: T = serde_json::from_str(&encoded0).expect("failed to decode");
    let encoded1 = serde_json::to_string(&decoded1).expect("failed to encode");

    assert_eq!(encoded0, encoded1);
    assert!(std::cmp::PartialEq::eq(&decoded0, &decoded1));
}}
"#,
            name
        );
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read")]
    Io(#[from] std::io::Error),
    #[error("invalid json")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// generate rust structs based on JSON data
pub fn translate<R, W>(r: &mut R, w: &mut W) -> Result<()>
where
    R: std::io::Read,
    W: std::io::Write,
{
    let v: Ty = serde_json::from_reader(r)?;

    let mut builder = TyBuilder::new();
    write!(w, "{}", builder.build("Root", v))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn it_works() {
        let json_str = r#"{
            "hello":"world",
            "foo":1,
            "some_null":[1,2,null],
            "null_some":[null,1,2],
            "null_inter":[null,1,2,null],
            "sign_mixed":[1,2,-3],
            "float_mixed":[1.0,-3,2],
            "null_mixed":[1.0,null,-2,null],
            "structs": [
                {"f1":1.0, "f2":"hello"},
                {"f1":-1, "f2":null},
                {"f3":false, "f2":"world"}
            ]
        }"#;

        let v: Ty = serde_json::from_str(&json_str).expect("Failed to decode");
        println!("{:?}", v);
    }

    fn write_test_runner<W: Write>(w: &mut W, filename: &str) -> Result<()> {
        write!(
            w,
            r#"{}
#[test]
fn test() {{
    let filename = "{}";
    let contents = std::fs::read_to_string(filename).expect("failed to read");
    test_runner::<Root>(&contents);
}}"#,
            TyBuilder::build_test_runner("test_runner"),
            filename
        )?;
        Ok(())
    }

    fn test_read_file(filename: &str) -> Result<()> {
        let mut file = File::open(filename)?;
        let mut out_file = File::create(filename.replace(".json", ".rs"))?;

        translate(&mut file, &mut out_file)?;
        write_test_runner(&mut out_file, filename)?;

        Ok(())
    }

    fn test_run_dir(dirname: &str) -> Result<()> {
        let paths = std::fs::read_dir(dirname)?;
        for path in paths {
            let filename = format!("{}", path?.path().display());
            if !filename.ends_with(".json") {
                continue;
            }
            test_read_file(&filename)?;
        }
        Ok(())
    }

    #[test]
    fn test_testcases() {
        test_run_dir("tests").expect("failed to handle testcases");
    }

    #[test]
    fn test_ident() {
        assert_eq!("hello", field_name("hello"));
        assert_eq!("field_01234", field_name("01234"));
        assert_eq!("field_struct", field_name("struct"));

        assert_eq!("hello_wor_ld", field_name("hello wor-ld"));
    }
}
