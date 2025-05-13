pub enum ProtocolVersion {
    V47
}

impl ProtocolVersion {
    pub fn id(self) -> i32 {
        match self {
            ProtocolVersion::V47 => 47,
        }
    }
}