use std::collections::HashMap;

pub trait PacketRegistry {
    fn id_to_label() -> HashMap<u8, &'static str>;
    fn label_to_id() -> HashMap<&'static str, u8>;
}

use std::collections::HashMap;
use super::PacketRegistry;

pub struct V47Registry;

impl PacketRegistry for V47Registry {
    fn id_to_label() -> HashMap<u8, &'static str> {
        HashMap::from([
            (0x00, "Handshake"),
            (0x00, "LoginStart"),
            (0x02, "LoginSuccess"),
            (0x03, "SetCompression"),
        ])
    }

    fn label_to_id() -> HashMap<&'static str, u8> {
        HashMap::from([
            ("Handshake", 0x00),
            ("LoginStart", 0x00),
            ("LoginSuccess", 0x02),
            ("SetCompression", 0x03),
        ])
    }
}