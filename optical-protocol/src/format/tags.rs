//! Traits which define common behavior for packets in different protocol states.

use std::fmt::Debug;

use downcast_rs::Downcast;

/// A packet that is sent in the void protocol state. (Serverbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait VoidPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

/// A packet that is sent in the status protocol state. (Serverbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait StatusPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

/// A packet that is sent in the login protocol state. (Serverbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait LoginPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

/// A packet that is sent in the play protocol state. (Serverbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait PlayPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

/// A packet that is sent in the void protocol state. (Clientbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait ClientVoidPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

/// A packet that is sent in the status protocol state. (Clientbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait ClientStatusPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

/// A packet that is sent in the login protocol state. (Clientbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait ClientLoginPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}

/// A packet that is sent in the play protocol state. (Clientbound)
#[typetag::serde(tag = "type", content = "adjacent")]
pub trait ClientPlayPacket: Debug + Send + Sync + Downcast {
    fn packet_id(&self) -> i32 {
        return i32::from_str_radix(self.typetag_name(), 10).unwrap();
    }
}
