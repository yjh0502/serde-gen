extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate error_chain;

use std::fmt;
use serde::de::*;


#[derive(Debug,Clone,PartialEq)]
pub enum Ty {
    Any,
    Unit,

    Bool,
    I,
    U,
    F,
    Char,
    Str(String),
    Bytes,

    None,
    Some(Box<Ty>),

    Seq(Box<Ty>),
    Map(Vec<(String, Ty)>),
}

impl std::ops::Add<Ty> for Ty {
    type Output = Ty;
    fn add(self, other: Ty) -> Ty {
        match (self, other) {
            // non-optional types
            (t, Ty::Unit) => t,
            (Ty::Unit, t) => t,

            // unsigned to signed
            (Ty::I, Ty::U) => Ty::I,
            (Ty::U, Ty::I) => Ty::I,

            // integer to float
            (Ty::I, Ty::F) => Ty::F,
            (Ty::F, Ty::I) => Ty::F,

            // integer to float
            (Ty::U, Ty::F) => Ty::F,
            (Ty::F, Ty::U) => Ty::F,

            (Ty::Str(_), Ty::Str(_)) => Ty::Str("".to_owned()),

            // merge struct
            (Ty::Map(mut m1), Ty::Map(m2)) => {
                for &(ref m2_name, ref m2_ty) in m2.iter() {
                    let mut found = false;
                    for tup in m1.iter_mut() {
                        if tup.0 == *m2_name {
                            *tup = (m2_name.clone(), tup.1.clone() + m2_ty.clone());
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        m1.push((m2_name.clone(), m2_ty.clone() + Ty::None));
                    }
                }

                for tup1 in m1.iter_mut() {
                    let mut found = false;
                    for tup2 in m2.iter() {
                        if tup1.0 == tup2.0 {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        *tup1 = (tup1.0.clone(), tup1.1.clone() + Ty::None)
                    }
                }
                Ty::Map(m1)
            }

            // nullable types
            (Ty::Some(t), Ty::None) => Ty::Some(t),
            (Ty::None, Ty::Some(t)) => Ty::Some(t),

            (Ty::Some(t1), t2) => Ty::Some(Box::new(*t1 + t2)),
            (t2, Ty::Some(t1)) => Ty::Some(Box::new(*t1 + t2)),

            (t, Ty::None) => Ty::Some(Box::new(t)),
            (Ty::None, t) => Ty::Some(Box::new(t)),

            // fallback to any
            (s, o) => if s == o { s } else { Ty::Any },
        }

        /*
        if self == Ty::Unit {
            return other;
        } else if other == Ty::Unit {
            return self;
        }

        // nullable types
        if self == Ty::None {
            return Ty::Some(Box::new(other));
        } else if other == Ty::None {
            return Ty::Some(Box::new(self));
        }

        if self == other { self } else { Ty::Any }
        */
    }
}

const RESERVED: &[&str] = &["as", "break", "const", "continue", "crate", "else", "enum", "extern",
                            "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
                            "mod", "move", "mut", "pub", "ref", "return", "Self", "self",
                            "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
                            "where", "while", "abstract", "alignof", "become", "box", "do",
                            "final", "macro", "offsetof", "override", "priv", "proc", "pure",
                            "sizeof", "typeof", "unsized", "virtual", "yield"];

fn field_name(name: &str) -> String {
    if RESERVED.contains(&name) {
        format!("field_{}", name)
    } else {
        name.to_owned()
    }
}

use std::collections::HashMap;

pub struct TyBuilder {
    names: HashMap<String, usize>,
    queue: Vec<(String, Vec<(String, Ty)>)>,
}

impl TyBuilder {
    pub fn new() -> Self {
        TyBuilder {
            names: HashMap::new(),
            queue: Vec::new(),
        }
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
                self.queue.push((name.clone(), m));
                name
            }

            // Any, Unit, Some,
            _ => unimplemented!(),
        }
    }

    pub fn build(&mut self, ty: Ty) -> String {
        let mut s = String::new();

        if let Ty::Map(v) = ty {
            let name = format!("Root");
            self.queue.push((name, v));
        }

        while let Some((name, def)) = self.queue.pop() {
            s.push_str(&format!("pub struct {} {{\n", name));

            for (name, ty) in def.into_iter() {
                s.push_str(&format!("    pub {}: {},\n",
                                   field_name(&name),
                                   self.ty_str(&name, ty)));
            }
            s.push_str("}\n\n");
        }
        s
    }
}


impl<'de> Deserialize<'de> for Ty {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Ty, D::Error>
        where D: serde::Deserializer<'de>
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Ty;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::Bool)
            }
            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::I)
            }
            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::U)
            }
            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::F)
            }
            fn visit_char<E>(self, _: char) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::Char)
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::Str(v.to_owned()))
            }
            fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::Bytes)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
                where E: Error
            {
                Ok(Ty::None)
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where D: Deserializer<'de>
            {
                Ok(Ty::Some(Box::new(Deserialize::deserialize(deserializer)?)))
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E>
                where E: Error
            {
                //XXX
                Ok(Ty::None)
            }

            /*
            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where D: Deserializer<'de>
            { unimplemented!() }
            */
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where A: SeqAccess<'de>
            {
                let mut ty = Ty::Unit;

                while let Some(elem) = seq.next_element()? {
                    ty = ty + elem;
                }
                Ok(Ty::Seq(Box::new(ty)))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where A: MapAccess<'de>
            {
                let mut v = Vec::new();

                while let Some((key, value)) = map.next_entry()? {
                    if let Ty::Str(s) = key {
                        v.push((s, value));
                    }
                }

                Ok(Ty::Map(v))
            }

            /*
            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where A: EnumAccess<'de>
            { unimplemented!() }
            */
        }

        deserializer.deserialize_any(ValueVisitor)
    }
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

    error_chain!{
        foreign_links {
            Io(std::io::Error);
            Json(serde_json::Error);
        }
    }

    fn test_read_file(filename: &str) -> Result<()> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let v: Ty = serde_json::from_str(&contents)?;
        println!("def: {:?}\n", v);

        let mut builder = TyBuilder::new();
        println!("code:\n{}\n", builder.build(v));
        Ok(())
    }

    #[test]
    fn test_wikipedia() {
        test_read_file("tests/wikipedia_changes.json").expect("failed to test file");
    }
}
