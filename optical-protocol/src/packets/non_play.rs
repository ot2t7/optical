pub mod void {
    pub mod serverbound {
        use serde::{Deserialize, Serialize};

        use crate::format::{tags::VoidPacket, types::VarInt};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Handshake {
            protocol_version: VarInt,
            server_address: String,
            server_port: u16,
            next_state: VarInt,
        }
        #[typetag::serde(name = "0")]
        impl VoidPacket for Handshake {}
    }
}

pub mod status {
    pub mod clientbound {}

    pub mod serverbound {}
}

pub mod login {
    pub mod clientbound {}

    pub mod serverbound {}
}
