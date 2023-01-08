use std::fmt::Debug;

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait VoidPacket: Debug + Send + Sync {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait StatusPacket: Debug + Send + Sync {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait LoginPacket: Debug + Send + Sync {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

#[typetag::serde(tag = "type", content = "adjacent")]
pub trait PlayPacket: Debug + Send + Sync {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}