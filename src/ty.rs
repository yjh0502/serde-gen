use std;
use serde;
use serde::de::*;

#[derive(Debug, Clone, PartialEq)]
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
    }
}

impl<'de> Deserialize<'de> for Ty {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Ty, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Ty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::Bool)
            }
            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::I)
            }
            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::U)
            }
            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::F)
            }
            fn visit_char<E>(self, _: char) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::Char)
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::Str(v.to_owned()))
            }
            fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::Bytes)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Ty::None)
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Ty::Some(Box::new(Deserialize::deserialize(deserializer)?)))
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
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
            where
                A: SeqAccess<'de>,
            {
                let mut ty = Ty::Unit;

                while let Some(elem) = seq.next_element()? {
                    ty = ty + elem;
                }
                Ok(Ty::Seq(Box::new(ty)))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
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
