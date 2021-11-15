use core::ops::Deref;
use std::fmt::Debug;
use std::{error::Error, fmt::Display};

use serde::de::{self};
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone)]
pub struct LenLimitedString<const S: usize> {
    size: usize,
    data: String,
}

impl<const S: usize> Default for LenLimitedString<S> {
    fn default() -> Self {
        Self {
            size: S,
            data: Default::default(),
        }
    }
}

impl<const S: usize> PartialEq for LenLimitedString<S> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

struct LenLimitedStringVisitor<const L: usize>;

#[derive(Debug)]
pub struct SizeError {
    real: usize,
    expect: usize,
}

impl<const S: usize> LenLimitedString<S> {
    fn new(s: String) -> Self {
        Self { size: S, data: s }
    }
}

impl<const S: usize> TryFrom<String> for LenLimitedString<S> {
    type Error = SizeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match (S, value.len()) {
            (ls, l) if ls >= l => Ok(Self::new(value)),
            (ls, rs) => Err(SizeError {
                real: rs,
                expect: ls,
            }),
        }
    }
}

impl<const S: usize> Into<String> for LenLimitedString<S> {
    fn into(self) -> String {
        self.data
    }
}
impl<const S: usize> From<&LenLimitedString<S>> for LenLimitedString<S> {
    fn from(d: &LenLimitedString<S>) -> Self {
        Self::new(d.data.clone())
    }
}

impl<const S: usize> Deref for LenLimitedString<S> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<const S: usize> Debug for LenLimitedString<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LenLimitedString")
            .field("data", &self.data)
            .field("maxSize", &self.size)
            .field("realSize", &self.data.len())
            .finish()
    }
}

impl<const L: usize> Serialize for LenLimitedString<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.data)
    }
}

impl<const L: usize> Visitor<'_> for LenLimitedStringVisitor<L> {
    type Value = LenLimitedString<L>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Cannot Load String From Src")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self.visit_string(v.to_string())
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        let res = LenLimitedString::<L>::try_from(v)
            .or_else(|e| Err(de::Error::custom(e.to_string())))?;

        Ok(res)
    }
}

impl<'de, const L: usize> Deserialize<'de> for LenLimitedString<L> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = LenLimitedStringVisitor::<L>;

        deserializer.deserialize_string(visitor)
    }
}

impl Display for SizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "size limit is {} but get size {}",
            self.expect, self.real
        )
    }
}

impl Error for SizeError {}


