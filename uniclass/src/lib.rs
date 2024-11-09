#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(feature = "serde")]
use std::marker::PhantomData;
use std::{num::ParseIntError, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uniclass {
    table: UniclassTable,
    codes: u32,
}

impl Uniclass {
    pub fn new(
        table: UniclassTable,
        group: u8,
        sub_group: Option<u8>,
        section: Option<u8>,
        object: Option<u8>,
    ) -> Self {
        let sub_group: u8 = sub_group.unwrap_or(0b10000000);
        let section: u8 = section.unwrap_or(0b10000000);
        let object: u8 = object.unwrap_or(0b10000000);
        let codes = u32::from_be_bytes([group, sub_group, section, object]);
        Uniclass { table, codes }
    }

    pub fn table(&self) -> UniclassTable {
        self.table
    }
    pub fn group(&self) -> u8 {
        self.codes.to_be_bytes()[0]
    }
    pub fn sub_group(&self) -> Option<u8> {
        if 0b10000000 & self.codes.to_be_bytes()[1] > 0 {
            None
        } else {
            Some(self.codes.to_be_bytes()[1])
        }
    }
    pub fn section(&self) -> Option<u8> {
        if 0b10000000 & self.codes.to_be_bytes()[2] > 0 {
            None
        } else {
            Some(self.codes.to_be_bytes()[2])
        }
    }
    pub fn object(&self) -> Option<u8> {
        if 0b10000000 & self.codes.to_be_bytes()[3] > 0 {
            None
        } else {
            Some(self.codes.to_be_bytes()[3])
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UniclassTable {
    Ac,
    Co,
    En,
    RK,
    SL,
    EF,
    Ss,
    Pr,
    TE,
    PM,
    Ma,
    PC,
    FI,
    Ro,
    Zz,
}

#[cfg(feature = "serde")]
impl Serialize for Uniclass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Uniclass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UniclassVisitor::new())
    }
}

impl FromStr for Uniclass {
    type Err = ParseUniclassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split('_');
        let table = {
            let table_str = segments
                .next()
                .ok_or(ParseUniclassError::InsufficientSegments)?;
            match table_str {
                "Ac" => UniclassTable::Ac,
                "Co" => UniclassTable::Co,
                "En" => UniclassTable::En,
                "SL" => UniclassTable::SL,
                "EF" => UniclassTable::EF,
                "Ss" => UniclassTable::Ss,
                "Pr" => UniclassTable::Pr,
                "TE" => UniclassTable::TE,
                "PM" => UniclassTable::PM,
                "FI" => UniclassTable::FI,
                "Ro" => UniclassTable::Ro,
                "Zz" => UniclassTable::Zz,
                "Ma" => UniclassTable::Ma,
                "PC" => UniclassTable::PC,
                "RK" => UniclassTable::RK,
                _ => return Err(ParseUniclassError::InvalidTable),
            }
        };
        let group = segments
            .next()
            .ok_or(ParseUniclassError::InsufficientSegments)?
            .parse()
            .map_err(ParseUniclassError::ParseIntError)?;
        let sub_group: u8 = if let Some(s) = segments.next() {
            s.parse().map_err(ParseUniclassError::ParseIntError)?
        } else {
            0b10000000
        };
        let section: u8 = if let Some(s) = segments.next() {
            s.parse().map_err(ParseUniclassError::ParseIntError)?
        } else {
            0b10000000
        };
        let object: u8 = if let Some(s) = segments.next() {
            s.parse().map_err(ParseUniclassError::ParseIntError)?
        } else {
            0b10000000
        };
        let codes = u32::from_be_bytes([group, sub_group, section, object]);
        Ok(Uniclass { table, codes })
    }
}

impl std::fmt::Display for Uniclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.table, self.group())?;
        if let Some(s) = self.sub_group() {
            write!(f, "_{}", s)?;
        }
        if let Some(s) = self.section() {
            write!(f, "_{}", s)?;
        }
        if let Some(s) = self.object() {
            write!(f, "_{}", s)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for UniclassTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UniclassTable::Ac => "Ac",
            UniclassTable::Co => "Co",
            UniclassTable::En => "En",
            UniclassTable::SL => "SL",
            UniclassTable::EF => "EF",
            UniclassTable::Ss => "Ss",
            UniclassTable::Pr => "Pr",
            UniclassTable::TE => "TE",
            UniclassTable::PM => "PM",
            UniclassTable::PC => "PC",
            UniclassTable::RK => "RK",
            UniclassTable::Ma => "Ma",
            UniclassTable::FI => "FI",
            UniclassTable::Ro => "Ro",
            UniclassTable::Zz => "Zz",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum ParseUniclassError {
    InvalidTable,
    InsufficientSegments,
    ParseIntError(ParseIntError),
}

impl std::fmt::Display for ParseUniclassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidTable => {
                write!(f, "InvalidTable")
            }
            Self::InsufficientSegments => {
                write!(f, "InsufficientSegments")
            }
            Self::ParseIntError(ref e) => {
                write!(f, "ParseIntError: {}", e)
            }
        }
    }
}

impl std::error::Error for ParseUniclassError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::InvalidTable => None,
            Self::InsufficientSegments => None,
            Self::ParseIntError(ref e) => e.source(),
        }
    }
}

#[cfg(feature = "serde")]
struct UniclassVisitor<T> {
    _t: PhantomData<T>,
}

#[cfg(feature = "serde")]
impl<T> UniclassVisitor<T> {
    fn new() -> Self {
        Self { _t: PhantomData }
    }
}

#[cfg(feature = "serde")]
impl<'de, T: FromStr> Visitor<'de> for UniclassVisitor<T>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A Uniclass code")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_size() {
        assert_eq!(std::mem::size_of::<Uniclass>(), 8);
    }

    #[test]
    fn decoding_uniclass() {
        let product_a: Uniclass = "Ss_25_20_15_16".parse().expect("test");
        let product_b: Uniclass = "Ss_25_20_50".parse().expect("test");
        let expected_a = Uniclass {
            table: UniclassTable::Ss,
            codes: u32::from_be_bytes([25, 20, 15, 16]),
        };
        let expected_b = Uniclass {
            table: UniclassTable::Ss,
            codes: u32::from_be_bytes([25, 20, 50, 128]),
        };
        assert_eq!(product_a, expected_a);
        assert_eq!(product_a.group(), 25);
        assert_eq!(product_a.sub_group(), Some(20));
        assert_eq!(product_a.section(), Some(15));
        assert_eq!(product_a.object(), Some(16));

        assert_eq!(product_b, expected_b);
        assert_eq!(product_b.group(), 25);
        assert_eq!(product_b.sub_group(), Some(20));
        assert_eq!(product_b.section(), Some(50));
        assert_eq!(product_b.object(), None);
    }
}
