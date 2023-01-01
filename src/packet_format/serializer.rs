use serde::{
    ser::{self, Error as SerdeError},
    Serialize,
};

use super::{
    error::Error,
    types::{write_var_int, write_var_long, MinecraftUuid, VarInt, VarLong},
};

pub struct Serializer {
    output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    return Ok(serializer.output);
}

fn write_size_or_index<T>(buf: &mut Serializer, value: T) -> Result<(), Error>
where
    T: TryInto<i32>,
    <T as TryInto<i32>>::Error: std::fmt::Debug,
{
    // Safe to unwrap, very unlikely an enum variant or a size/index would
    // be higher than an i32's max value.
    write_var_int(&mut buf.output, value.try_into().unwrap())
        .map_err(|e| Error::Message(e.to_string()))?;
    return Ok(());
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.output.push(1);
        } else {
            self.output.push(0);
        }
        return Ok(());
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push(v);
        return Ok(());
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_be_bytes());
        return Ok(());
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.to_string().as_bytes());
        return Ok(());
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v.as_bytes());
        return Ok(());
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.output.extend(v);
        return Ok(());
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        return self.serialize_bool(false);
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        return self.serialize_bool(true);
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        return Ok(());
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        return Ok(());
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write_size_or_index(self, variant_index)?;
        return Ok(());
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        return value.serialize(self);
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        write_size_or_index(self, variant_index)?;
        return value.serialize(self);
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(value) = len {
            write_size_or_index(self, value)?;
        }
        return Ok(self);
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        return Ok(self);
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        return Ok(self);
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        write_size_or_index(self, variant_index)?;
        return Ok(self);
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        return Ok(self);
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        return Ok(self);
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        write_size_or_index(self, variant_index)?;
        return Ok(self);
    }
}

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let buf = vec![0u8; 5];
        write_var_int(&mut buf, self.value).map_err(|e| SerdeError::custom(e.to_string()))?;
        serializer.serialize_bytes(&buf)?;
        return Ok(());
    }
}
impl Serialize for VarLong {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let buf = vec![0u8; 10];
        write_var_long(&mut buf, self.value).map_err(|e| SerdeError::custom(e.to_string()))?;
        serializer.serialize_bytes(&buf)?;
    }
}
impl Serialize for MinecraftUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.0.as_bytes();
        serializer.serialize_bytes(bytes);
        return Ok(());
    }
}
