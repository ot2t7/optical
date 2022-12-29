use std::io::{Cursor, Read};

use super::error::Error;
use super::types::{read_string, read_var_int, read_var_long, VarInt};
use serde::de::MapAccess;
use serde::{de::SeqAccess, Deserialize};

pub struct Deserializer<'de> {
    input: &'de mut Cursor<Vec<u8>>,
    /// Did this deserializer already process the packet id?
    id_read: bool,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de mut Cursor<Vec<u8>>, id_read: bool) -> Self {
        return Deserializer { input, id_read };
    }
}

pub fn from_bytes<'a, T>(input: &'a mut Cursor<Vec<u8>>) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    read_var_int(input).map_err(|_| Error::MalformedVarInt)?; // packet length
    read_var_int(input).map_err(|_| Error::MalformedVarInt)?; // packet id
    let mut deserializer = Deserializer::from_bytes(input, true);
    let t = T::deserialize(&mut deserializer)?;
    return Ok(t);
}

/// When the return is a `Box<dyn Packet>` or likewise
pub fn from_bytes_generic<'a, T>(input: &'a mut Cursor<Vec<u8>>) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    read_var_int(input).map_err(|_| Error::MalformedVarInt)?; // packet length
    let mut deserializer = Deserializer::from_bytes(input, false);
    let t = T::deserialize(&mut deserializer)?;
    return Ok(t);
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return Err(Error::ParsingAny);
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut byte = [0u8];
        self.input
            .read_exact(&mut byte)
            .map_err(|_| Error::MalformedBool)?;
        return visitor.visit_bool(byte[0] == 1);
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = [0u8; 4];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedI32)?;
        return visitor.visit_i32(i32::from_be_bytes(bytes));

        /*
        return match read_var_int(self.input) {
            Ok(n) => visitor.visit_i32(n.value),
            Err(_) => Err(Error::MalformedVarInt),
        }; */
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return match read_var_long(self.input) {
            Ok(n) => visitor.visit_i64(n.value),
            Err(_) => Err(Error::MalformedVarInt),
        };
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut byte = [0u8];
        match self.input.read_exact(&mut byte) {
            Ok(_) => visitor.visit_u8(byte[0]),
            Err(_) => return Err(Error::NoMoreBytes),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = [0u8; 2];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedU16)?;
        return visitor.visit_u16(u16::from_be_bytes(bytes));
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("str!");
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.id_read == false {
            self.id_read = true;
            let id = match read_var_int(self.input) {
                Ok(n) => Ok(n.value),
                Err(_) => Err(Error::MalformedVarInt),
            }?;
            return visitor.visit_string(id.to_string());
        }
        return match read_string(self.input) {
            Ok(n) => visitor.visit_string(n),
            Err(_) => Err(Error::MalformedVarInt),
        };
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return visitor.visit_seq(Flatten::new(self));
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return visitor.visit_seq(Flatten::new(self));
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if name == "VarInt" && fields == ["value", "size"] {}
        return self.deserialize_seq(visitor);
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}

/// All structs, maps, and arrays are represented as sequences
/// with no data for keys in the minecraft packet format. A
/// `Flatten` describes this behavior through its access
/// implementations.
struct Flatten<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Flatten<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Flatten { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for Flatten<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        return seed.deserialize(&mut *self.de).map(Some);
    }
}
