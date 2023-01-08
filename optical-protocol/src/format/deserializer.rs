use std::io::{Cursor, Read};

use super::error::Error;
use super::types::{read_string, read_var_int, read_var_long, MinecraftUuid, VarInt, VarLong};
use serde::de::Error as SerdeError;
use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};
use serde::{de::SeqAccess, Deserialize};

pub struct Deserializer<'de> {
    input: &'de mut Cursor<Vec<u8>>,
    /// Is the next String element supposed to be deserialized
    /// as a var int?
    need_id_read: bool,
    /// Is the next String element supposed to be deserialized
    /// as a var int, and then mapped into a variant name?
    need_id_variant: Option<&'static [&'static str]>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de mut Cursor<Vec<u8>>, need_id_read: bool) -> Self {
        return Deserializer {
            input,
            need_id_read,
            need_id_variant: None,
        };
    }
}

pub fn from_bytes<'a, T>(input: &'a mut Cursor<Vec<u8>>) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    read_var_int(input).map_err(|_| Error::MalformedVarInt)?; // packet length
    read_var_int(input).map_err(|_| Error::MalformedVarInt)?; // packet id
    let mut deserializer = Deserializer::from_bytes(input, false);
    let t = T::deserialize(&mut deserializer)?;
    return Ok(t);
}

/// When the return is a `Box<dyn Packet>` or likewise
pub fn from_bytes_generic<'a, T>(input: &'a mut Cursor<Vec<u8>>) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    read_var_int(input).map_err(|_| Error::MalformedVarInt)?; // packet length
    let mut deserializer = Deserializer::from_bytes(input, true);
    let t = T::deserialize(&mut deserializer)?;
    return Ok(t);
}

impl<'de, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return Err(Error::AnyType);
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
        let mut bytes = [0u8; 1];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedI8)?;
        return visitor.visit_i8(i8::from_be_bytes(bytes));
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = [0u8; 2];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedI16)?;
        return visitor.visit_i16(i16::from_be_bytes(bytes));
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
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = [0u8; 8];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedI64)?;
        return visitor.visit_i64(i64::from_be_bytes(bytes));
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
        let mut bytes = [0u8; 4];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedU32)?;
        return visitor.visit_u32(u32::from_be_bytes(bytes));
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = [0u8; 8];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedU64)?;
        return visitor.visit_u64(u64::from_be_bytes(bytes));
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = [0u8; 4];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedF32)?;
        return visitor.visit_f32(f32::from_be_bytes(bytes));
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut bytes = [0u8; 8];
        self.input
            .read_exact(&mut bytes)
            .map_err(|_| Error::MalformedF64)?;
        return visitor.visit_f64(f64::from_be_bytes(bytes));
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return Err(Error::CharType);
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.need_id_read == true {
            // Need to read some kind of var int id
            self.need_id_read = false;
            if let Some(variants) = self.need_id_variant {
                // Need to read some kind of var int
                // id and map into an enum variant
                self.need_id_variant = None;
                let variant_index: usize = read_var_int(self.input)
                    .map_err(|_| Error::MalformedVarInt)?
                    .value
                    .try_into()
                    .map_err(|_| Error::MalformedVarInt)?;
                return visitor.visit_string(variants[variant_index].to_string());
            }
            let id = read_var_int(self.input)
                .map_err(|_| Error::MalformedVarInt)?
                .value;
            return visitor.visit_string(id.to_string());
        }
        // Just need to read a string;
        return visitor.visit_string(read_string(self.input).map_err(|_| Error::MalformedString)?);
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return self.deserialize_byte_buf(visitor);
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut rest_of_input = vec![];
        self.input
            .read_to_end(&mut rest_of_input)
            .map_err(|_| Error::NoMoreBytes)?;
        return visitor.visit_byte_buf(rest_of_input);
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut byte = [0u8];
        self.input
            .read_exact(&mut byte)
            .map_err(|_| Error::MalformedBool)?;
        let is_some = byte[0] == 1;
        if is_some {
            return visitor.visit_some(self);
        } else {
            return visitor.visit_none();
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return visitor.visit_unit();
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return visitor.visit_unit();
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return visitor.visit_newtype_struct(self);
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return visitor.visit_seq(Flatten::new(self, true)?);
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return visitor.visit_seq(Flatten::new(self, false)?);
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return self.deserialize_tuple(len, visitor);
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return Err(Error::MapType);
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return self.deserialize_tuple(fields.len(), visitor);
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // The var int id which will be requested next will come in
        // as a string. Communicate this to the parser to make sure
        // it's parsed as a normal var int, and not a string.
        self.need_id_read = true;
        self.need_id_variant = Some(variants);
        return visitor.visit_enum(Enum::new(self));
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return self.deserialize_string(visitor);
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        return Err(Error::AnyType);
    }

    // Hints to types like UUID's that this format is not self-describing/human readable
    fn is_human_readable(&self) -> bool {
        false
    }
}

/// All structs, maps, arrays are represented as sequences
/// with no data for keys in the minecraft packet format. A
/// `Flatten` describes this behavior through its access
/// implementations.
struct Flatten<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    size: Option<VarInt>,
}

impl<'a, 'de> Flatten<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, read_length: bool) -> Result<Self, Error> {
        let size;
        if read_length {
            size = read_var_int(de.input).ok();
        } else {
            size = None;
        }
        return Ok(Flatten { de, size });
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

    fn size_hint(&self) -> Option<usize> {
        // Return None if the size is negative somehow
        return self.size.clone()?.value.try_into().ok();
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        return Enum { de };
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        println!("Yippee! Time to deserialize variant!");
        let val = seed.deserialize(&mut *self.de)?;
        return Ok((val, self));
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        return seed.deserialize(self.de);
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        return serde::de::Deserializer::deserialize_tuple(self.de, len, visitor);
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        return serde::de::Deserializer::deserialize_tuple(self.de, fields.len(), visitor);
    }
}

// Special deserialization logic for varints

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VarIntVisitor;

        impl<'de> Visitor<'de> for VarIntVisitor {
            type Value = Result<VarInt, Error>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a var int")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                pub const CONTINUE_BIT: u8 = 0b10000000;
                let mut buf = [0u8; 5];
                let mut filled = 0;
                loop {
                    let next_byte: u8 = match seq.next_element()? {
                        Some(n) => n,
                        None => return Ok(Err(Error::MalformedVarInt)),
                    };
                    buf[filled] = next_byte;
                    filled += 1;
                    if next_byte & CONTINUE_BIT == 0 {
                        break;
                    }
                }
                return Ok(read_var_int(&mut Cursor::new(buf.to_vec()))
                    .map_err(|_| Error::MalformedVarInt));
            }
        }

        // The len is technically unknown, so just say its zero.
        let res = deserializer.deserialize_tuple(0, VarIntVisitor)?;
        return Ok(res.map_err(|e| SerdeError::custom(e))?);
    }
}

// Special deserialization logic for var longs

impl<'de> Deserialize<'de> for VarLong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VarLongVisitor;

        impl<'de> Visitor<'de> for VarLongVisitor {
            type Value = Result<VarLong, Error>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a var long")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                pub const CONTINUE_BIT: u8 = 0b10000000;
                let mut buf = [0u8; 10];
                let mut filled = 0;
                loop {
                    let next_byte: u8 = match seq.next_element()? {
                        Some(n) => n,
                        None => return Ok(Err(Error::MalformedVarLong)),
                    };
                    buf[filled] = next_byte;
                    filled += 1;
                    if next_byte & CONTINUE_BIT == 0 {
                        break;
                    }
                }
                return Ok(read_var_long(&mut Cursor::new(buf.to_vec()))
                    .map_err(|_| Error::MalformedVarLong));
            }
        }

        // The len is technically unknown, so just say its zero.
        let res = deserializer.deserialize_tuple(0, VarLongVisitor)?;
        return Ok(res.map_err(|e| SerdeError::custom(e))?);
    }
}

impl<'de> Deserialize<'de> for MinecraftUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = uuid::serde::compact::deserialize(deserializer)?;
        return Ok(MinecraftUuid(inner));
    }
}
