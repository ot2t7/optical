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
    pub mod clientbound {
        use crate::format::tags::StatusPacket;
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct StatusResponse {
            pub json_response: String,
        }
        #[typetag::serde(name = "0")]
        impl StatusPacket for StatusResponse {}

        #[derive(Serialize, Deserialize, Debug)]
        pub struct PingResponse {
            pub payload: i64,
        }
        #[typetag::serde(name = "1")]
        impl StatusPacket for PingResponse {}
    }

    pub mod serverbound {
        use crate::format::tags::StatusPacket;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct StatusRequest {}
        #[typetag::serde(name = "0")]
        impl StatusPacket for StatusRequest {}

        #[derive(Serialize, Deserialize, Debug)]
        pub struct PingRequest {
            payload: i64,
        }
        #[typetag::serde(name = "1")]
        impl StatusPacket for PingRequest {}
    }
}

pub mod login {
    pub mod clientbound {
        use crate::format::{
            tags::LoginPacket,
            types::{MinecraftUuid, VarInt},
        };
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Disconnect {
            pub reason: String,
        }
        #[typetag::serde(name = "0")]
        impl LoginPacket for Disconnect {}

        #[derive(Serialize, Deserialize, Debug)]
        pub struct EncryptionRequest {
            pub server_id: String,
            pub public_key: Vec<u8>,
            pub verify_token: Vec<u8>,
        }
        #[typetag::serde(name = "1")]
        impl LoginPacket for EncryptionRequest {}

        #[derive(Serialize, Deserialize, Debug)]
        pub enum LoginSuccessProperties {
            None,
            One {
                name: String,
            },
            Two {
                name: String,
                value: String,
            },
            Three {
                name: String,
                value: String,
                is_signed: bool,
            },
            Four {
                name: String,
                value: String,
                signature: Option<String>,
            },
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct LoginSuccess {
            uuid: MinecraftUuid,
            username: String,
            properties: LoginSuccessProperties,
        }
        #[typetag::serde(name = "2")]
        impl LoginPacket for LoginSuccess {}

        #[derive(Serialize, Deserialize, Debug)]
        pub struct SetCompression {
            pub threshold: VarInt,
        }
        #[typetag::serde(name = "3")]
        impl LoginPacket for SetCompression {}

        #[derive(Serialize, Deserialize, Debug)]
        pub struct LoginPluginRequest {
            message_id: VarInt,
            channel: String,
            data: Byte,
        }
        #[typetag::serde(name = "4")]
        impl LoginPacket for LoginPluginRequest {}
    }

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
