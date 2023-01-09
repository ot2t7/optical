pub mod void {
    pub mod serverbound {
        use serde::{Deserialize, Serialize};

        use crate::format::{tags::VoidPacket, types::VarInt};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Handshake {
            pub protocol_version: VarInt,
            pub server_address: String,
            pub server_port: u16,
            pub next_state: VarInt,
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

    pub mod serverbound {
        use serde::{Deserialize, Serialize};

        use crate::format::{tags::LoginPacket, types::MinecraftUuid};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct LoginStart {
            pub name: String,
            pub player_uuid: Option<MinecraftUuid>,
        }
        #[typetag::serde(name = "0")]
        impl LoginPacket for LoginStart {}
    }
}
