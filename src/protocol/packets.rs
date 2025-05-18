use std::{io::{self, Read}, net::TcpStream};
use fastnbt::{from_bytes, to_bytes, Value};
use uuid::{Uuid};

use super::server;

#[derive(Debug)]
pub enum IDS {
    // -- Handshaking -- //
    Handshake,

    // -- Login -- //
    DisconnectLogin,
    EncryptionRequest,
    LoginSuccess,
    SetCompressionLogin,

    LoginStart,
    EncryptionResponse,

    // -- Play -- //
    KeepAliveClientbound,
    JoinGame,
    ChatMessageClientbound,
    TimeUpdate,
    EntityEquipment,
    SpawnPosition,
    UpdateHealth,
    Respawn,
    PlayerPositionAndLookClientbound,
    HeldItemChangeClientbound,
    UseBed,
    AnimationClientbound,
    SpawnPlayer,
    CollectItem,
    SpawnObject,
    SpawnMob,
    SpawnPainting,
    SpawnExperienceOrb,
    EntityVelocity,
    DestroyEntities,
    Entity,
    EntityRelativeMove,
    EntityLook,
    EntityLookAndRelativeMove,
    EntityTeleport,
    EntityHeadLook,
    EntityStatus,
    AttachEntity,
    EntityMetadata,
    EntityEffect,
    RemoveEntityEffect,
    SetExperience,
    EntityProperties,
    ChunkData,
    MultiBlockChange,
    BlockChange,
    BlockAction,
    BlockBreakAnimation,
    MapChunkBulk,
    Explosion,
    Effect,
    SoundEffect,
    Particle,
    ChangeGameState,
    SpawnGlobalEntity,
    OpenWindow,
    CloseWindowClientbound,
    SetSlot,
    WindowItems,
    WindowProperty,
    ConfirmTransactionClientbound,
    UpdateSignClientbound,
    Map,
    UpdateBlockEntity,
    OpenSignEditor,
    Statistics,
    PlayerListItem,
    PlayerAbilitiesClientbound,
    TabCompleteClientbound,
    ScoreboardObjective,
    UpdateScore,
    DisplayScoreboard,
    Teams,
    PluginMessageClientbound,
    DisconnectPlayClientbound,
    ServerDifficulty,
    CombatEvent,
    Camera,
    WorldBorder,
    Title,
    SetCompressionPlay,
    PlayerListHeaderAndFooter,
    ResourcePackSend,
    UpdateEntityNBT,

    KeepAliveServerbound,
    ChatMessageServerbound,
    UseEntity,
    Player,
    PlayerPosition,
    PlayerLook,
    PlayerPositionAndLookServerbound,
    PlayerDigging,
    PlayerBlockPlacement,
    HeldItemChangeServerbound,
    AnimationServerbound,
    EntityAction,
    SteerVehicle,
    CloseWindowServerbound,
    ClickWindow,
    ConfirmTransactionServerbound,
    CreativeInventoryAction,
    EnchantItem,
    UpdateSignServerbound,
    PlayerAbilitiesServerbound,
    TabCompleteServerbound,
    ClientSettings,
    ClientStatus,
    PluginMessageServerbound,
    Spectate,
    ResoucePackStatus,
}

impl IDS {
    pub fn parse(&self) -> i32 {
        match self {
            IDS::Handshake => 0x00,

            IDS::DisconnectLogin => 0x00,
            IDS::EncryptionRequest => 0x01,
            IDS::LoginSuccess => 0x02,
            IDS::SetCompressionLogin => 0x03,

            IDS::LoginStart => 0x00,
            IDS::EncryptionResponse => 0x01,

            IDS::KeepAliveClientbound => 0x00,
            IDS::JoinGame => 0x01,
            IDS::ChatMessageClientbound => 0x02,
            IDS::TimeUpdate => 0x03,
            IDS::EntityEquipment => 0x04,
            IDS::SpawnPosition => 0x05,
            IDS::UpdateHealth => 0x06,
            IDS::Respawn => 0x07,
            IDS::PlayerPositionAndLookClientbound => 0x08,
            IDS::HeldItemChangeClientbound => 0x09,
            IDS::UseBed => 0x0A,
            IDS::AnimationClientbound => 0x0B,
            IDS::SpawnPlayer => 0x0C,
            IDS::CollectItem => 0x0D,
            IDS::SpawnObject => 0x0E,
            IDS::SpawnMob => 0x0F,
            IDS::SpawnPainting => 0x10,
            IDS::SpawnExperienceOrb => 0x11,
            IDS::EntityVelocity => 0x12,
            IDS::DestroyEntities => 0x13,
            IDS::Entity => 0x14,
            IDS::EntityRelativeMove => 0x15,
            IDS::EntityLook => 0x16,
            IDS::EntityLookAndRelativeMove => 0x17,
            IDS::EntityTeleport => 0x18,
            IDS::EntityHeadLook => 0x19,
            IDS::EntityStatus => 0x1A,
            IDS::AttachEntity => 0x1B,
            IDS::EntityMetadata => 0x1C,
            IDS::EntityEffect => 0x1D,
            IDS::RemoveEntityEffect => 0x1E,
            IDS::SetExperience => 0x1F,
            IDS::EntityProperties => 0x20,
            IDS::ChunkData => 0x21,
            IDS::MultiBlockChange => 0x22,
            IDS::BlockChange => 0x23,
            IDS::BlockAction => 0x24,
            IDS::BlockBreakAnimation => 0x25,
            IDS::MapChunkBulk => 0x26,
            IDS::Explosion => 0x27,
            IDS::Effect => 0x28,
            IDS::SoundEffect => 0x29,
            IDS::Particle => 0x2A,
            IDS::ChangeGameState => 0x2B,
            IDS::SpawnGlobalEntity => 0x2C,
            IDS::OpenWindow => 0x2D,
            IDS::CloseWindowClientbound => 0x2E,
            IDS::SetSlot => 0x2F,
            IDS::WindowItems => 0x30,
            IDS::WindowProperty => 0x31,
            IDS::ConfirmTransactionClientbound => 0x32,
            IDS::UpdateSignClientbound => 0x33,
            IDS::Map => 0x34,
            IDS::UpdateBlockEntity => 0x35,
            IDS::OpenSignEditor => 0x36,
            IDS::Statistics => 0x37,
            IDS::PlayerListItem => 0x38,
            IDS::PlayerAbilitiesClientbound => 0x39,
            IDS::TabCompleteClientbound => 0x3A,
            IDS::ScoreboardObjective => 0x3B,
            IDS::UpdateScore => 0x3C,
            IDS::DisplayScoreboard => 0x3D,
            IDS::Teams => 0x3E,
            IDS::PluginMessageClientbound => 0x3F,
            IDS::DisconnectPlayClientbound => 0x40,
            IDS::ServerDifficulty => 0x41,
            IDS::CombatEvent => 0x42,
            IDS::Camera => 0x43,
            IDS::WorldBorder => 0x44,
            IDS::Title => 0x45,
            IDS::SetCompressionPlay => 0x46,
            IDS::PlayerListHeaderAndFooter => 0x47,
            IDS::ResourcePackSend => 0x48,
            IDS::UpdateEntityNBT => 0x49,

            IDS::KeepAliveServerbound => 0x00,
            IDS::ChatMessageServerbound => 0x01,
            IDS::UseEntity => 0x02,
            IDS::Player => 0x03,
            IDS::PlayerPosition => 0x04,
            IDS::PlayerLook => 0x05,
            IDS::PlayerPositionAndLookServerbound => 0x06,
            IDS::PlayerDigging => 0x07,
            IDS::PlayerBlockPlacement => 0x08,
            IDS::HeldItemChangeServerbound => 0x09,
            IDS::AnimationServerbound => 0x0A,
            IDS::EntityAction => 0x0B,
            IDS::SteerVehicle => 0x0C,
            IDS::CloseWindowServerbound => 0x0D,
            IDS::ClickWindow => 0x0E,
            IDS::ConfirmTransactionServerbound => 0x0F,
            IDS::CreativeInventoryAction => 0x10,
            IDS::EnchantItem => 0x11,
            IDS::UpdateSignServerbound => 0x12,
            IDS::PlayerAbilitiesServerbound => 0x13,
            IDS::TabCompleteServerbound => 0x14,
            IDS::ClientSettings => 0x15,
            IDS::ClientStatus => 0x16,
            IDS::PluginMessageServerbound => 0x17,
            IDS::Spectate => 0x18,
            IDS::ResoucePackStatus => 0x19,
        }
    }
    pub fn from_login_clientbound_id(id: i32) -> Option<Self> {
        match id {
            0x00 => Some(IDS::DisconnectLogin),
            0x01 => Some(IDS::EncryptionRequest),
            0x02 => Some(IDS::LoginSuccess),
            0x03 => Some(IDS::SetCompressionLogin),
            _ => None,
        }
    }
    pub fn from_play_clientbound_id(id: i32) -> Option<Self> {
        match id {
            0x00 => Some(IDS::KeepAliveClientbound),
            0x01 => Some(IDS::JoinGame),
            0x02 => Some(IDS::ChatMessageClientbound),
            0x03 => Some(IDS::TimeUpdate),
            0x04 => Some(IDS::EntityEquipment),
            0x05 => Some(IDS::SpawnPosition),
            0x06 => Some(IDS::UpdateHealth),
            0x07 => Some(IDS::Respawn),
            0x08 => Some(IDS::PlayerPositionAndLookClientbound),
            0x09 => Some(IDS::HeldItemChangeClientbound),
            0x0A => Some(IDS::UseBed),
            0x0B => Some(IDS::AnimationClientbound),
            0x0C => Some(IDS::SpawnPlayer),
            0x0D => Some(IDS::CollectItem),
            0x0E => Some(IDS::SpawnObject),
            0x0F => Some(IDS::SpawnMob),
            0x10 => Some(IDS::SpawnPainting),
            0x11 => Some(IDS::SpawnExperienceOrb),
            0x12 => Some(IDS::EntityVelocity),
            0x13 => Some(IDS::DestroyEntities),
            0x14 => Some(IDS::Entity),
            0x15 => Some(IDS::EntityRelativeMove),
            0x16 => Some(IDS::EntityLook),
            0x17 => Some(IDS::EntityLookAndRelativeMove),
            0x18 => Some(IDS::EntityTeleport),
            0x19 => Some(IDS::EntityHeadLook),
            0x1A => Some(IDS::EntityStatus),
            0x1B => Some(IDS::AttachEntity),
            0x1C => Some(IDS::EntityMetadata),
            0x1D => Some(IDS::EntityEffect),
            0x1E => Some(IDS::RemoveEntityEffect),
            0x1F => Some(IDS::SetExperience),
            0x20 => Some(IDS::EntityProperties),
            0x21 => Some(IDS::ChunkData),
            0x22 => Some(IDS::MultiBlockChange),
            0x23 => Some(IDS::BlockChange),
            0x24 => Some(IDS::BlockAction),
            0x25 => Some(IDS::BlockBreakAnimation),
            0x26 => Some(IDS::MapChunkBulk),
            0x27 => Some(IDS::Explosion),
            0x28 => Some(IDS::Effect),
            0x29 => Some(IDS::SoundEffect),
            0x2A => Some(IDS::Particle),
            0x2B => Some(IDS::ChangeGameState),
            0x2C => Some(IDS::SpawnGlobalEntity),
            0x2D => Some(IDS::OpenWindow),
            0x2E => Some(IDS::CloseWindowClientbound),
            0x2F => Some(IDS::SetSlot),
            0x30 => Some(IDS::WindowItems),
            0x31 => Some(IDS::WindowProperty),
            0x32 => Some(IDS::ConfirmTransactionClientbound),
            0x33 => Some(IDS::UpdateSignClientbound),
            0x34 => Some(IDS::Map),
            0x35 => Some(IDS::UpdateBlockEntity),
            0x36 => Some(IDS::OpenSignEditor),
            0x37 => Some(IDS::Statistics),
            0x38 => Some(IDS::PlayerListItem),
            0x39 => Some(IDS::PlayerAbilitiesClientbound),
            0x3A => Some(IDS::TabCompleteClientbound),
            0x3B => Some(IDS::ScoreboardObjective),
            0x3C => Some(IDS::UpdateScore),
            0x3D => Some(IDS::DisplayScoreboard),
            0x3E => Some(IDS::Teams),
            0x3F => Some(IDS::PluginMessageClientbound),
            0x40 => Some(IDS::DisconnectPlayClientbound),
            0x41 => Some(IDS::ServerDifficulty),
            0x42 => Some(IDS::CombatEvent),
            0x43 => Some(IDS::Camera),
            0x44 => Some(IDS::WorldBorder),
            0x45 => Some(IDS::Title),
            0x46 => Some(IDS::SetCompressionPlay),
            0x47 => Some(IDS::PlayerListHeaderAndFooter),
            0x48 => Some(IDS::ResourcePackSend),
            0x49 => Some(IDS::UpdateEntityNBT),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Byte(pub i8);
#[derive(Debug, Clone, Copy)]
pub struct UnsignedByte(pub u8);
#[derive(Debug, Clone, Copy)]
pub struct Short(pub i16);
#[derive(Debug, Clone, Copy)]
pub struct UnsignedShort(pub u16);
#[derive(Debug, Clone, Copy)]
pub struct Int(pub i32);
#[derive(Debug, Clone, Copy)]
pub struct Long(pub i64);
#[derive(Debug, Clone, Copy)]
pub struct VarLong(pub i64);
#[derive(Debug, Clone, Copy)]
pub struct Float(pub f32);
#[derive(Debug, Clone, Copy)]
pub struct Double(pub f64);
#[derive(Debug, Clone, Copy)]
pub struct VarInt(pub i32);
#[derive(Debug, Clone, Copy)]
pub struct Boolean(pub bool);
#[derive(Debug, Clone)]
pub struct VarString(pub String);
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct Angle(pub f32);  // stored in degrees
#[derive(Debug, Clone, Copy)]
pub struct UUID(pub [u8; 16]);
#[derive(Debug, Clone)]
pub struct Slot {
    pub present: bool,
    pub item_id: i16,
    pub item_count: u8,
    pub item_damage: i16,
    pub nbt: Option<Value>,
}
#[derive(Debug, Clone)]
pub enum MetadataValue {
    Byte(i8),
    VarInt(i32),
    Float(f32),
    String(String),
    Chat(String),
    Slot(Vec<u8>),
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(i64),
    OptionalUuid(Option<Uuid>),
    BlockId(i32),
    Unknown(u8), // fallback
}
#[derive(Debug, Clone)]
pub struct MetadataEntry {
    pub index: u8,
    pub value: MetadataValue,
}
#[derive(Debug, Clone)]
pub struct VarIntArray {
    pub elements: Vec<VarInt>,
}

impl VarIntArray {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        let (VarInt(length), mut offset) = VarInt::parse(data)?;
        let mut elements = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let (val, size) = VarInt::parse(&data[offset..])?;
            offset += size;
            elements.push(val);
        }

        Some((Self { elements }, offset))
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        VarInt(self.elements.len() as i32).write(&mut buf);
        for v in &self.elements {
            v.write(&mut buf);
        }
        buf
    }
}

#[derive(Debug, Clone)]
pub struct EntityMetadata {
    pub entries: Vec<MetadataEntry>
}

impl EntityMetadata {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        let mut offset = 0;
        let mut entries = Vec::new();

        while offset < data.len() {
            let key = data[offset];
            offset += 1;

            if key == 0x7F {
                break;
            }

            let index = key & 0x1F;
            let type_id = key >> 5;

            let value = match type_id {
                0 => {
                    let val = data.get(offset).ok_or("Unexpected EOF reading Byte").unwrap() as &u8;
                    offset += 1;
                    MetadataValue::Byte(*val as i8)
                }
                1 => {
                    let (val, size) = VarInt::parse(&data[offset..]).ok_or("Failed to parse VarInt").unwrap();
                    offset += size;
                    MetadataValue::VarInt(val.0)
                }
                2 => {
                    let val = f32::from_be_bytes(data.get(offset..offset + 4).ok_or("Unexpected EOF reading Float").unwrap().try_into().unwrap());
                    offset += 4;
                    MetadataValue::Float(val)
                }
                3 | 4 => {
                    let (s, size) = VarString::parse(&data[offset..]).ok_or("Failed to parse String").unwrap();
                    offset += size;
                    if type_id == 3 {
                        MetadataValue::String(s.0)
                    } else {
                        MetadataValue::Chat(s.0)
                    }
                }
                5 => {
                    // Simplified slot parsing; you may expand this
                    let length = 5;
                    let val = data.get(offset..offset + length).ok_or("EOF slot").unwrap().to_vec();
                    offset += length;
                    MetadataValue::Slot(val)
                }
                6 => {
                    let val = data.get(offset).ok_or("EOF bool").unwrap();
                    offset += 1;
                    MetadataValue::Boolean(*val != 0)
                }
                7 => {
                    let x = f32::from_be_bytes(data.get(offset..offset + 4).ok_or("EOF rot x").unwrap().try_into().unwrap());
                    offset += 4;
                    let y = f32::from_be_bytes(data.get(offset..offset + 4).ok_or("EOF rot y").unwrap().try_into().unwrap());
                    offset += 4;
                    let z = f32::from_be_bytes(data.get(offset..offset + 4).ok_or("EOF rot z").unwrap().try_into().unwrap());
                    offset += 4;
                    MetadataValue::Rotation(x, y, z)
                }
                8 => {
                    let val = i64::from_be_bytes(data.get(offset..offset + 8).ok_or("EOF pos").unwrap().try_into().unwrap());
                    offset += 8;
                    MetadataValue::Position(val)
                }
                9 => {
                    let present = *data.get(offset).ok_or("EOF present bool").unwrap() != 0;
                    offset += 1;
                    let uuid = if present {
                        let bytes = data.get(offset..offset + 16).ok_or("EOF uuid").unwrap();
                        offset += 16;
                        Some(Uuid::from_slice(bytes).map_err(|_| "Invalid UUID").unwrap())
                    } else {
                        None
                    };
                    MetadataValue::OptionalUuid(uuid)
                }
                10 => {
                    let (val, size) = VarInt::parse(&data[offset..]).ok_or("Failed to parse block id").unwrap();
                    offset += size;
                    MetadataValue::BlockId(val.0)
                }
                unknown => MetadataValue::Unknown(unknown),
            };

            entries.push(MetadataEntry { index, value });
        }

        Some((Self { entries }, offset))
    }
}

impl Slot {
    pub fn parse(buf: &[u8]) -> Option<(Self, usize)> {
        let mut offset = 0;

        // Presence byte
        let present = *buf.get(offset)? != 0;
        offset += 1;

        if !present {
            return Some((
                Slot {
                    present: false,
                    item_id: 0,
                    item_count: 0,
                    item_damage: 0,
                    nbt: None,
                },
                offset,
            ));
        }

        // Item ID (2 bytes)
        let item_id = i16::from_be_bytes(buf.get(offset..offset + 2)?.try_into().ok()?);
        offset += 2;

        // Count (1 byte)
        let item_count = *buf.get(offset)?;
        offset += 1;

        // Damage (2 bytes)
        let item_damage = i16::from_be_bytes(buf.get(offset..offset + 2)?.try_into().ok()?);
        offset += 2;

        // NBT: if first byte is 0, it's absent
        if *buf.get(offset)? == 0 {
            offset += 1;
            return Some((
                Slot {
                    present: true,
                    item_id,
                    item_count,
                    item_damage,
                    nbt: None,
                },
                offset,
            ));
        }

        // Try parsing NBT
        let nbt_start = offset;
        let nbt_result = from_bytes::<Value>(&buf[nbt_start..]);

        match nbt_result {
            Ok(nbt_value) => {
                // Re-serialize it to get the size
                let nbt_bytes = fastnbt::to_bytes(&nbt_value).ok()?;
                offset += nbt_bytes.len();

                Some((
                    Slot {
                        present: true,
                        item_id,
                        item_count,
                        item_damage,
                        nbt: Some(nbt_value),
                    },
                    offset,
                ))
            }
            Err(_) => None,
        }
    }
    pub fn write(&self, out: &mut Vec<u8>) {
        out.push(self.present as u8);

        if self.present {
            out.extend_from_slice(&self.item_id.to_be_bytes());

            out.push(self.item_count);

            out.extend_from_slice(&self.item_damage.to_be_bytes());
            if let Some(nbt_value) = &self.nbt {
                let nbt_bytes = to_bytes(nbt_value).unwrap();
                out.extend_from_slice(&nbt_bytes);
            } else {
                // No NBT = single 0 byte
                out.push(0);
            }
        }
    }
}

impl UUID {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 16 { return None; }
        Some((UUID(data[0..16].try_into().unwrap()), 16))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0);
    }
}

impl Angle {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        data.get(0).map(|b| {
            let degrees = (*b as f32) * 360.0 / 256.0;
            (Angle(degrees), 1)
        })
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        let byte = ((self.0 / 360.0) * 256.0) as u8;
        out.push(byte);
    }
}

impl Position {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 8 { return None; }
        let val = i64::from_be_bytes(data[0..8].try_into().unwrap());

        let x = (val >> 38) as i32;
        let y = ((val >> 26) & 0xFFF) as i32;
        let z = (val << 38 >> 38) as i32;

        Some((Position { x, y, z }, 8))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        let mut val = ((self.x as i64 & 0x3FFFFFF) << 38)
                    | ((self.y as i64 & 0xFFF) << 26)
                    | (self.z as i64 & 0x3FFFFFF);

        out.extend_from_slice(&val.to_be_bytes());
    }
}

impl VarString {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        let (len, len_size) = VarInt::parse(data)?;
        let len = len.0 as usize;

        let start = len_size;
        let end = start + len;
        if data.len() < end {
            return None;
        }

        let str_data = &data[start..end];
        let str_value = std::str::from_utf8(str_data).ok()?.to_owned();

        Some((VarString(str_value), len_size + len))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        let bytes = self.0.as_bytes();
        VarInt(bytes.len() as i32).write(out);
        out.extend_from_slice(bytes);
    }
}

impl VarInt {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        let mut num = 0i32;
        let mut shift = 0;
        let mut consumed = 0;

        for byte in data.iter().take(5) {
            let val = (byte & 0x7F) as i32;
            num |= val << shift;
            consumed += 1;

            if byte & 0x80 == 0 {
                return Some((VarInt(num), consumed));
            }

            shift += 7;
        }

        None // too long or incomplete
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        let mut value = self.0 as u32;
        loop {
            if value & !0x7F == 0 {
                out.push(value as u8);
                return;
            } else {
                out.push(((value & 0x7F) | 0x80) as u8);
                value >>= 7;
            }
        }
    }
}

impl Boolean {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        data.get(0).map(|b| (Boolean(*b != 0), 1))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.push(if self.0 { 1 } else { 0 });
    }
}

impl Byte {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        data.get(0).map(|b| (Byte(*b as i8), 1))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.push(self.0 as u8);
    }
}

impl UnsignedByte {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        data.get(0).map(|b| (UnsignedByte(*b), 1))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.push(self.0);
    }
}

impl Short {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 2 { return None; }
        Some((Short(i16::from_be_bytes([data[0], data[1]])), 2))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_be_bytes());
    }
}

impl UnsignedShort {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 2 { return None; }
        Some((UnsignedShort(u16::from_be_bytes([data[0], data[1]])), 2))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_be_bytes());
    }
}

impl Int {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 4 { return None; }
        Some((Int(i32::from_be_bytes(data[0..4].try_into().unwrap())), 4))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_be_bytes());
    }
}

impl Long {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 8 { return None; }
        Some((Long(i64::from_be_bytes(data[0..8].try_into().unwrap())), 8))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_be_bytes());
    }
}

impl VarLong {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        let mut num: i64 = 0;
        let mut shift = 0;
        let mut consumed = 0;

        for byte in data.iter().take(10) {
            let val = (byte & 0x7F) as i64;
            num |= val << shift;
            consumed += 1;
            if byte & 0x80 == 0 {
                return Some((VarLong(num), consumed));
            }
            shift += 7;
        }
        None
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        let mut value = self.0 as u64;
        loop {
            if value & !0x7F == 0 {
                out.push(value as u8);
                return;
            } else {
                out.push(((value & 0x7F) | 0x80) as u8);
                value >>= 7;
            }
        }
    }
}

impl Float {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 4 { return None; }
        let bits = u32::from_be_bytes(data[0..4].try_into().unwrap());
        Some((Float(f32::from_bits(bits)), 4))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_bits().to_be_bytes());
    }
}

impl Double {
    pub fn parse(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 8 { return None; }
        let bits = u64::from_be_bytes(data[0..8].try_into().unwrap());
        Some((Double(f64::from_bits(bits)), 8))
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.0.to_bits().to_be_bytes());
    }
}

pub fn read_varint_from_stream(stream: &mut TcpStream) -> io::Result<i32> {
    let mut num_read = 0;
    let mut result = 0i32;

    loop {
        let mut byte = [0u8; 1];
        stream.read_exact(&mut byte)?;

        let value = (byte[0] & 0b0111_1111) as i32;
        result |= value << (7 * num_read);

        num_read += 1;
        if num_read > 5 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "VarInt too big"));
        }

        if byte[0] & 0b1000_0000 == 0 {
            break;
        }
    }

    Ok(result)
}

pub trait ClientboundPacket: Sized {
    fn parse(data: &[u8]) -> Result<Self, String>;
}

#[macro_export]
macro_rules! clientbound_packet {
    (
        $name:ident, $id:expr, {
            $( $field_name:ident : $field_type:ty ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub struct $name {
            $( pub $field_name: $field_type ),*
        }

        impl ClientboundPacket for $name {
            fn parse(data: &[u8]) -> Result<Self, String> {
                let mut offset = 0;
                $(
                    let ($field_name, size) = <$field_type>::parse(&data[offset..])
                        .ok_or_else(|| format!("Failed to parse field {}", stringify!($field_name)))?;
                    offset += size;
                )*
                Ok($name {
                    $( $field_name ),*
                })
            }
        }
    };
}

clientbound_packet!{
    LoginSuccess, IDS::LoginSuccess, {
        uuid: VarString,
        username: VarString
    }
}

clientbound_packet!{
    SetCompressionLogin, IDS::SetCompressionLogin, {
        threshold: VarInt
    }
}

clientbound_packet!{
    KeepAliveClientbound, IDS::KeepAliveClientbound, {
        keep_alive_id: VarInt
    }
}

clientbound_packet!{
    JoinGame, IDS::JoinGame, {
        entity_id: Int,
        game_mode: UnsignedByte,
        dimension: Byte,
        difficulty: UnsignedByte,
        max_players: UnsignedByte,
        level_type: VarString,
        reduced_debug_info: Boolean
    }
}

clientbound_packet!{
    ChatMessageClientbound, IDS::ChatMessageClientbound, {
        chat: VarString,
        position: Byte
    }
}

clientbound_packet!{
    TimeUpdate, IDS::TimeUpdate, {
        world_age: Long,
        time_of_day: Long
    }
}

clientbound_packet!{
    EntityEquipment, IDS::EntityEquipment, {
        entity_id: VarInt,
        slot: Short,
        item: Slot
    }
}

clientbound_packet!{
    SpawnPosition, IDS::SpawnPosition, {
        location: Position
    }
}

clientbound_packet!{
    UpdateHealth, IDS::UpdateHealth, {
        health: Float,
        food: VarInt,
        food_saturation: Float
    }
}

clientbound_packet!{
    Respawn, IDS::Respawn, {
        dimension: Int,
        difficulty: UnsignedByte,
        gamemode: UnsignedByte,
        level_type: VarString
    }
}

clientbound_packet!{
    PlayerPositionAndLookClientbound, IDS::PlayerPositionAndLookClientbound, {
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        flags: Byte
    }
}

clientbound_packet!{
    HeldItemChangeClientbound, IDS::HeldItemChangeClientbound, {
        slot: Byte
    }
}

clientbound_packet!{
    UseBed, IDS::UseBed, {
        entity_id: VarInt,
        location: Position
    }
}

clientbound_packet!{
    AnimationClientbound, IDS::AnimationClientbound, {
        entity_id: VarInt,
        animation: UnsignedByte
    }
}

clientbound_packet!{
    SpawnPlayer, IDS::SpawnPlayer, {
        entity_id: VarInt,
        player_uuid: UUID,
        x: Int,
        y: Int,
        z: Int,
        yaw: Angle,
        pitch: Angle,
        current_item: Short,
        metadata: EntityMetadata
    }
}

pub struct SpawnObject {
    entity_id: VarInt,
    _type: Byte,
    x: Int,
    y: Int,
    z: Int,
    pitch: Angle,
    yaw: Angle,
    data: Int,
    velocity_x: Short,
    velocity_y: Short,
    velocity_z: Short
}

impl ClientboundPacket for SpawnObject {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (entity_id, entity_id_size) = VarInt::parse(&data[offset..]).unwrap();
        offset += entity_id_size;
        let (_type, _type_size) = Byte::parse(&data[offset..]).unwrap();
        offset += _type_size;
        let (x, x_size) = Int::parse(&data[offset..]).unwrap();
        offset += x_size;
        let (y, y_size) = Int::parse(&data[offset..]).unwrap();
        offset += y_size;
        let (z, z_size) = Int::parse(&data[offset..]).unwrap();
        offset += z_size;
        let (pitch, pitch_size) = Angle::parse(&data[offset..]).unwrap();
        offset += pitch_size;
        let (yaw, yaw_size) = Angle::parse(&data[offset..]).unwrap();
        offset += yaw_size;
        let (_data, data_size) = Int::parse(&data[offset..]).unwrap();
        offset += data_size;
        let mut velocity_x: Short = Short(0);
        let mut velocity_y: Short = Short(0);
        let mut velocity_z: Short = Short(0);
        if(_data.0 != 0) {
            let (_velocity_x, velocity_x_size) = Short::parse(&data[offset..]).unwrap();
            velocity_x = _velocity_x;
            offset += velocity_x_size;
            let (_velocity_y, velocity_y_size) = Short::parse(&data[offset..]).unwrap();
            velocity_y = _velocity_y;
            offset += velocity_y_size;
            let (_velocity_z, velocity_z_size) = Short::parse(&data[offset..]).unwrap();
            velocity_z = _velocity_z;
            offset += velocity_z_size;
        }
        Ok(Self {entity_id, _type, x, y, z, pitch, yaw, data: _data, velocity_x, velocity_y, velocity_z})
    }
}

clientbound_packet!{
    SpawnMob, IDS::SpawnMob, {
        entity_id: VarInt,
        _type: UnsignedByte,
        x: Int,
        y: Int,
        z: Int,
        yaw: Angle,
        pitch: Angle,
        head_pitch: Angle,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short,
        metadata: EntityMetadata
    }
}

clientbound_packet!{
    SpawnPainting, IDS::SpawnPainting, {
        entity_id: VarInt,
        title: VarString,
        location: Position,
        direction: Short
    }
}

clientbound_packet!{
    SpawnExperienceOrb, IDS::SpawnExperienceOrb, {
        entity_id: VarInt,
        x: Int,
        y: Int,
        z: Int,
        count: Short
    }
}

clientbound_packet!{
    EntityVelocity, IDS::EntityVelocity, {
        entity_id: VarInt,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short
    }
}

clientbound_packet!{
    DestroyEntities, IDS::DestroyEntities, {
        count: VarInt,
        entity_ids: VarIntArray
    }
}

clientbound_packet!{
    Entity, IDS::Entity, {
        entity_id: VarInt
    }
}

clientbound_packet!{
    EntityRelativeMove, IDS::EntityRelativeMove, {
        entity_id: VarInt,
        delta_x: Byte,
        delta_y: Byte,
        delta_z: Byte,
        on_ground: Boolean
    }
}

clientbound_packet!{
    EntityLook, IDS::EntityLook, {
        entity_id: VarInt,
        yaw: Angle,
        pitch: Angle,
        on_ground: Boolean
    }
}

clientbound_packet!{
    EntityLookAndRelativeMove, IDS::EntityLookAndRelativeMove, {
        entity_id: VarInt,
        delta_x: Byte,
        delta_y: Byte,
        delta_z: Byte,
        yaw: Angle,
        pitch: Angle,
        on_ground: Boolean
    }
}

clientbound_packet!{
    EntityTeleport, IDS::EntityTeleport, {
        entity_id: VarInt,
        x: Int,
        y: Int,
        z: Int,
        yaw: Angle,
        pitch: Angle,
        on_ground: Boolean
    }
}

clientbound_packet!{
    EntityHeadLook, IDS::EntityHeadLook, {
        entity_id: VarInt,
        head_yaw: Angle,
    }
}

clientbound_packet!{
    EntityStatus, IDS::EntityStatus, {
        entity_id: Int,
        entity_status: Byte
    }
}

clientbound_packet!{
    AttachEntity, IDS::AttachEntity, {
        entity_id: Int,
        vehicle_id: Int,
        leash: Boolean
    }
}

clientbound_packet!{
    EntityMetadataPacket, IDS::EntityMetadata, {
        entity_id: VarInt,
        metadata: EntityMetadata
    }
}

clientbound_packet!{
    EntityEffect, IDS::EntityEffect, {
        entity_id: VarInt,
        effect_id: Byte,
        amplifier: Byte,
        duration: VarInt,
        hide_particles: Boolean
    }
}

clientbound_packet!{
    RemoveEntityEffect, IDS::RemoveEntityEffect, {
        entity_id: VarInt,
        effect_id: Byte
    }
}

clientbound_packet!{
    SetExperience, IDS::SetExperience, {
        experience_bar: Float,
        level: VarInt,
        total_experience: VarInt
    }
}

pub  struct EntityProperties {
    entity_id: VarInt,
    properties: Vec<(VarString, Double, Vec<(UUID, Double, Byte)>)>
}

impl ClientboundPacket for EntityProperties {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (entity_id, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let (number_of_properties, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut properties: Vec<(VarString, Double, Vec<(UUID, Double, Byte)>)> = Vec::new();
        for _i in 0..number_of_properties.0 {
            let (key, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            let (value, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            let (number_of_modifiers, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            let mut modifiers: Vec<(UUID, Double, Byte)> = Vec::new();
            for _j in 0..number_of_modifiers.0 {
                let (uuid, size) = UUID::parse(&data[offset..]).unwrap();
                offset += size;
                let (amount, size) = Double::parse(&data[offset..]).unwrap();
                offset += size;
                let (operation, size) = Byte::parse(&data[offset..]).unwrap();
                offset += size;
                modifiers.push((uuid, amount, operation));
            }
            properties.push((key, value, modifiers));
        }
        Ok(Self { entity_id, properties })
    }
}

pub struct ChunkData {
    chunk_x: Int,
    chunk_z: Int,
    ground_up_continuous: Boolean,
    primary_bit_mask: UnsignedShort,
    data: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>
}

impl ClientboundPacket for ChunkData {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (chunk_x, size) = Int::parse(&data[offset..]).unwrap();
        offset += size;
        let (chunk_z, size) = Int::parse(&data[offset..]).unwrap();
        offset += size;
        let (ground_up_continuous, size) = Boolean::parse(&data[offset..]).unwrap();
        offset += size;
        let (primary_bit_mask, size) = UnsignedShort::parse(&data[offset..]).unwrap();
        offset += size;
        let (data_size, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut chunk_data: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = Vec::new();
        for _i in 0..primary_bit_mask.0.count_ones() {
            let block_ids: Vec<u8> = data[offset..offset + 4096].to_vec();
            offset += 4096;
            let block_metadata: Vec<u8> = data[offset..offset + 2048].to_vec();
            offset += 2048;
            let block_light: Vec<u8> = data[offset..offset + 2048].to_vec();
            offset += 2048;
            let sky_light: Vec<u8> = data[offset..offset + 2048].to_vec();
            offset += 2048;
            chunk_data.push((block_ids, block_metadata, block_light, sky_light));
        }
        Ok(Self { chunk_x, chunk_z, ground_up_continuous, primary_bit_mask, data: chunk_data })
    }
}

pub struct MultiBlockChange {
    chunk_x: Int,
    chunk_z: Int,
    records: Vec<(UnsignedByte, UnsignedByte, VarInt)>
}

impl ClientboundPacket for MultiBlockChange {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (chunk_x, size) = Int::parse(&data[offset..]).unwrap();
        offset += size;
        let (chunk_z, size) = Int::parse(&data[offset..]).unwrap();
        offset += size;
        let (record_count, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut records: Vec<(UnsignedByte, UnsignedByte, VarInt)> = Vec::new();
        for i in 0..record_count.0 {
            let (horizontal_position, size) = UnsignedByte::parse(&data[offset..]).unwrap();
            offset += size;
            let (y_coordinate, size) = UnsignedByte::parse(&data[offset..]).unwrap();
            offset += size;
            let (block_id, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            records.push((horizontal_position, y_coordinate, block_id));
        }
        Ok(Self { chunk_x, chunk_z, records })
    }
}

clientbound_packet!{
    BlockChange, IDS::BlockChange, {
        location: Position,
        block_id: VarInt
    }
}

clientbound_packet!{
    BlockAction, IDS::BlockAction, {
        location: Position,
        byte_1: UnsignedByte,
        byte_2: UnsignedByte,
        block_type: VarInt
    }
}

clientbound_packet!{
    BlockBreakAnimation, IDS::BlockBreakAnimation, {
        entity_id: VarInt,
        location: Position,
        destroy_stage: Byte
    }
}

pub struct MapChunkBulk {
    sky_light_sent: Boolean,
    chunk_meta: Vec<(Int, Int, UnsignedShort)>,
    chunk_data: Vec<Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>>
}

impl ClientboundPacket for MapChunkBulk {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (sky_light_sent, size) = Boolean::parse(&data[offset..]).unwrap();
        offset += size;
        let (chunk_column_count, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut chunk_meta: Vec<(Int, Int, UnsignedShort)> = Vec::new();
        for _i in 0..chunk_column_count.0 {
            let (chunk_x, size) = Int::parse(&data[offset..]).unwrap();
            offset += size;
            let (chunk_y, size) = Int::parse(&data[offset..]).unwrap();
            offset += size;
            let (primary_bit_mask, size) = UnsignedShort::parse(&data[offset..]).unwrap();
            offset += size;
            chunk_meta.push((chunk_x, chunk_y, primary_bit_mask));
        }
        let mut chunk_data: Vec<Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>> = Vec::new();
        for i in 0..chunk_column_count.0 {
            let mut single_chunk: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = Vec::new();
            for _j in 0..chunk_meta.get(i as usize).unwrap().2.0.count_ones() {
                let block_ids: Vec<u8> = data[offset..offset + 4096].to_vec();
                offset += 4096;
                let block_metadata: Vec<u8> = data[offset..offset + 2048].to_vec();
                offset += 2048;
                let block_light: Vec<u8> = data[offset..offset + 2048].to_vec();
                offset += 2048;
                let mut sky_light: Vec<u8> = Vec::new();
                if sky_light_sent.0 {
                    sky_light = data[offset..offset + 2048].to_vec();
                    offset += 2048;
                }
                single_chunk.push((block_ids, block_metadata, block_light, sky_light));
            }
            chunk_data.push(single_chunk);
        }
        Ok(Self { sky_light_sent, chunk_meta, chunk_data })
    }
}

pub struct Explosion {
    x: Float,
    y: Float,
    z: Float,
    radius: Float,
    records: Vec<(Byte, Byte, Byte)>,
    player_motion_x: Float,
    player_motion_y: Float,
    player_motion_z: Float
}

impl ClientboundPacket for Explosion {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (x, size) = Float::parse(&data[offset..]).unwrap();
        offset += size;
        let (y, size) = Float::parse(&data[offset..]).unwrap();
        offset += size;
        let (z, size) = Float::parse(&data[offset..]).unwrap();
        offset += size;
        let (radius, size) = Float::parse(&data[offset..]).unwrap();
        offset += size;
        let (record_count, size) = Int::parse(&data[offset..]).unwrap();
        offset += size;
        let mut records: Vec<(Byte, Byte, Byte)> = Vec::new();
        for i in 0..record_count.0 {
            let (damage_x, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            let (damage_y, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            let (damage_z, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            records.push((damage_x, damage_y, damage_z));
        }
        let (player_motion_x, size) = Float::parse(&data[offset..]).unwrap();
        offset += size;
        let (player_motion_y, size) = Float::parse(&data[offset..]).unwrap();
        offset += size;
        let (player_motion_z, _) = Float::parse(&data[offset..]).unwrap();
        Ok(Self { x, y, z, radius, records, player_motion_x, player_motion_y, player_motion_z })
    }
}

clientbound_packet!{
    Effect, IDS::Effect, {
        effect_id: Int,
        location: Position,
        data: Int,
        disable_relative_volume: Boolean
    }
}

clientbound_packet!{
    SoundEffect, IDS::SoundEffect, {
        sound_name: VarString,
        effect_position_x: Int,
        effect_position_y: Int,
        effect_position_z: Int,
        volume: Float,
        pitch: UnsignedByte
    }
}

clientbound_packet!{
    Particle, IDS::Particle, {
        particle_id: Int,
        long_distance: Boolean,
        x: Float,
        y: Float,
        z: Float,
        offset_x: Float,
        offset_y: Float,
        offset_z: Float,
        particle_data: Float,
        particle_count: Int,
        data: VarIntArray
    }
}

clientbound_packet!{
    ChangeGameState, IDS::ChangeGameState, {
        reason: UnsignedByte,
        value: Float
    }
}

clientbound_packet!{
    SpawnGlobalEntity, IDS::SpawnGlobalEntity, {
        entity_id: VarInt,
        _type: Byte,
        x: Int,
        y: Int,
        z: Int
    }
}

pub struct OpenWindow {
    window_id: UnsignedByte,
    window_type: VarString,
    window_title: VarString,
    number_of_slots: UnsignedByte,
    entity_id: Int
}

impl ClientboundPacket for OpenWindow {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (window_id, size) = UnsignedByte::parse(&data[offset..]).unwrap();
        offset += size;
        let (window_type, size) = VarString::parse(&data[offset..]).unwrap();
        offset += size;
        let (window_title, size) = VarString::parse(&data[offset..]).unwrap();
        offset += size;
        let (number_of_slots, size) = UnsignedByte::parse(&data[offset..]).unwrap();
        offset += size;
        let mut entity_id = Int(0);
        if window_type.0 == "EnityHorse" {
            let (_entity_id, size) = Int::parse(&data[offset..]).unwrap();
            entity_id = _entity_id;
        }
        Ok(Self { window_id, window_type, window_title, number_of_slots, entity_id })
    }
}

clientbound_packet!{
    CloseWindowClientbound, IDS::CloseWindowClientbound, {
        window_id: UnsignedByte
    }
}

clientbound_packet!{
    SetSlot, IDS::SetSlot, {
        window_id: Byte,
        slot: Short,
        slot_data: Slot
    }
}

pub struct WindowItems {
    pub window_id: UnsignedByte,
    pub slots: Vec<Slot>
}

impl ClientboundPacket for WindowItems {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (window_id, size) = UnsignedByte::parse(&data[offset..]).unwrap();
        offset += size;
        let (count, size) = Short::parse(&data[offset..]).unwrap();
        offset += size;
        let mut slots: Vec<Slot> = Vec::new();
        for i in 0..count.0 {
            let (slot, size) = Slot::parse(&data[offset..]).unwrap();
            offset += size;
            slots.push(slot);
        }
        Ok(Self { window_id, slots })
    }
}

clientbound_packet!{
    WindowProperty, IDS::WindowProperty, {
        window_id: UnsignedByte,
        property: Short,
        value: Short
    }
}

clientbound_packet!{
    ConfirmTransactionClientbound, IDS::ConfirmTransactionClientbound, {
        window_id: Byte,
        action_number: Short,
        accepted: Boolean
    }
}

clientbound_packet!{
    UpdateSignClientbound, IDS::UpdateSignClientbound, {
        location: Position,
        line_1: VarString,
        line_2: VarString,
        line_3: VarString,
        line_4: VarString
    }
}

pub struct Map {
    pub item_damage: VarInt,
    pub scale: Byte,
    pub icon: Vec<(Byte, Byte, Byte)>,
    pub columns: Byte,
    pub rows: Byte,
    pub x: Byte,
    pub z: Byte,
    pub data: Vec<UnsignedByte>
}

impl ClientboundPacket for Map {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (item_damage, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let (scale, size) = Byte::parse(&data[offset..]).unwrap();
        offset += size;
        let (icon_count, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut icon: Vec<(Byte, Byte, Byte)> = Vec::new();
        for i in 0..icon_count.0 {
            let (direction_and_type, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            let (x, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            let (z, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            icon.push((direction_and_type, x, z));
        }
        let (columns, size) = Byte::parse(&data[offset..]).unwrap();
        offset += size;
        let mut rows = Byte(0);
        let mut x = Byte(0);
        let mut z = Byte(0);
        let mut _data: Vec<UnsignedByte> = Vec::new();
        if columns.0 > 0 {
            let (_rows, size) = Byte::parse(&data[offset..]).unwrap();
            rows = _rows;
            offset += size;
            let (_x, size) = Byte::parse(&data[offset..]).unwrap();
            x = _x;
            offset += size;
            let (_z, size) = Byte::parse(&data[offset..]).unwrap();
            z = _z;
            offset += size;
            let (length, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            for j in 0..length.0 {
                let (byte, size) = UnsignedByte::parse(&data[offset..]).unwrap();
                offset += size;
                _data.push(byte);
            }
        }
        Ok(Self { item_damage, scale, icon, columns, rows, x, z, data: _data })
    }
}

pub struct UpdateBlockEntity {
    pub location: Position,
    pub action: UnsignedByte,
    pub nbt_data: Value
}

impl ClientboundPacket for UpdateBlockEntity {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (location, size) = Position::parse(&data[offset..]).unwrap();
        offset += size;
        let (action, size) = UnsignedByte::parse(&data[offset..]).unwrap();
        offset += size;
        let nbt_raw = &data[offset..];
        let nbt: fastnbt::Value = fastnbt::from_bytes(nbt_raw).unwrap();
        Ok(Self { location, action, nbt_data: nbt })
    }
}

clientbound_packet!{
    OpenSignEditor, IDS::OpenSignEditor, {
        location: Position
    }
}

pub struct Statistics {
    pub statistics: Vec<(VarString, VarInt)>
}

impl ClientboundPacket for Statistics {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (count, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut statistics: Vec<(VarString, VarInt)> = Vec::new();
        for i in 0..count.0 {
            let (name, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            let (value, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            statistics.push((name, value));
        }
        Ok(Self { statistics })
    }
}

pub struct PlayerListItem {
    pub action: VarInt,
    pub players: Vec<(UUID /* Player UUID */, (VarString /* Name */, Vec<(VarString /* Name */, VarString /* Value */, Boolean /* Is Signed */, VarString /* Signature */)> /* Properties */, VarInt /* Gamemode */, VarInt /* Ping */, Boolean /* Has Display Name */, VarString /* Display Name */), VarInt /* Gamemode */, VarInt /* Ping */, (Boolean /* Has Display Name */, VarString /* Display Name */))>
}

impl ClientboundPacket for PlayerListItem {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (action, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let (number_of_players, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut players: Vec<(UUID, (VarString, Vec<(VarString, VarString, Boolean, VarString)>, VarInt, VarInt, Boolean, VarString), VarInt, VarInt, (Boolean, VarString))> = Vec::new();
        for i in 0..number_of_players.0 {
            let (uuid, size) = UUID::parse(&data[offset..]).unwrap();
            offset += size;
            let mut name = VarString("".to_string());
            let mut properties: Vec<(VarString, VarString, Boolean, VarString)> = Vec::new();
            let mut gamemode = VarInt(0);
            let mut ping = VarInt(0);
            let mut has_display_name = Boolean(false);
            let mut display_name = VarString("".to_string());
            if action.0 == 0 {
                let (_name, size) = VarString::parse(&data[offset..]).unwrap();
                name = _name;
                offset += size;
                let (number_of_properties, size) = VarInt::parse(&data[offset..]).unwrap();
                offset += size;
                for i in 0..number_of_properties.0 {
                    let (property_name, size) = VarString::parse(&data[offset..]).unwrap();
                    offset += size;
                    let (property_value, size) = VarString::parse(&data[offset..]).unwrap();
                    offset += size;
                    let (property_is_signed, size) = Boolean::parse(&data[offset..]).unwrap();
                    offset += size;
                    let mut property_signature = VarString("".to_string());
                    if property_is_signed.0 {
                        let (_property_signature, size) = VarString::parse(&data[offset..]).unwrap();
                        property_signature = _property_signature;
                        offset += size;
                    }
                    properties.push((property_name, property_value, property_is_signed, property_signature));
                }
                let (_gamemode, size) = VarInt::parse(&data[offset..]).unwrap();
                gamemode = _gamemode;
                offset += size;
                let (_ping, size) = VarInt::parse(&data[offset..]).unwrap();
                ping = _ping;
                offset += size;
                let (_has_display_name, size) = Boolean::parse(&data[offset..]).unwrap();
                has_display_name = _has_display_name;
                offset += size;
                if has_display_name.0 {
                    let (_display_name, size) = VarString::parse(&data[offset..]).unwrap();
                    display_name = _display_name;
                    offset += size;
                }
            }
            if action.0 == 1 {
                let (_gamemode, size) = VarInt::parse(&data[offset..]).unwrap();
                gamemode = _gamemode;
                offset += size;
            }
            if action.0 == 2 {
                let (_ping, size) = VarInt::parse(&data[offset..]).unwrap();
                ping = _ping;
                offset += size;
            }
            if action.0 == 3 {
                let (_has_display_name, size) = Boolean::parse(&data[offset..]).unwrap();
                has_display_name = _has_display_name;
                offset += size;
                if has_display_name.0 {
                    let (_display_name, size) = VarString::parse(&data[offset..]).unwrap();
                    display_name = _display_name;
                    offset += size;
                }
            }
            players.push((uuid, (name, properties, gamemode, ping, has_display_name, display_name.clone()), gamemode, ping, (has_display_name, display_name)));
        }
        Ok(Self { action, players })
    }
}

clientbound_packet!{
    PlayerAbilitiesClientbound, IDS::PlayerAbilitiesClientbound, {
        flags: Byte,
        flying_speed: Float,
        field_of_view_modifier: Float
    }
}

pub struct TabCompleteClientbound {
    pub matches: Vec<VarString>
}

impl ClientboundPacket for TabCompleteClientbound {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (count, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut matches: Vec<VarString> = Vec::new();
        for i in 0..count.0 {
            let (_match, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            matches.push(_match);
        }
        Ok(Self { matches })
    }
}

pub struct ScoreboardObjective {
    pub objective_name: VarString,
    pub mode: Byte,
    pub objective_value: VarString,
    pub _type: VarString
}

impl ClientboundPacket for ScoreboardObjective {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (objective_name, size) = VarString::parse(&data[offset..]).unwrap();
        offset += size;
        let (mode, size) = Byte::parse(&data[offset..]).unwrap();
        offset += size;
        let mut objective_value = VarString("".to_string());
        let mut _type = VarString("".to_string());
        if mode.0 == 0 || mode.0 == 2 {
            let (_objective_value, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            objective_value = _objective_value;
            let (__type, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            _type = __type;
        }
        Ok(Self { objective_name, mode, objective_value, _type })
    }
}

pub struct UpdateScore {
    score_name: VarString,
    action: Byte,
    objective_name: VarString,
    value: VarInt
}

impl ClientboundPacket for UpdateScore {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (score_name, size) = VarString::parse(&data[offset..]).unwrap();
        offset += size;
        let (action, size) = Byte::parse(&data[offset..]).unwrap();
        offset += size;
        let (objective_name, size) = VarString::parse(&data[offset..]).unwrap();
        offset += size;
        let mut value = VarInt(0);
        if action.0 != 1 {
            let (_value, _) = VarInt::parse(&data[offset..]).unwrap();
            value = _value;
        }
        Ok(Self { score_name, action, objective_name, value })
    }
}

clientbound_packet!{
    DisplayScoreboard, IDS::DisplayScoreboard, {
        position: Byte,
        score_name: VarString
    }
}

pub struct Teams {
    pub team_name: VarString,
    pub mode: Byte,
    pub team_display_name: VarString,
    pub team_prefix: VarString,
    pub team_suffix: VarString,
    pub friendly_fire: Byte,
    pub name_tag_visibility: VarString,
    pub color: Byte,
    pub players: Vec<VarString>
}

impl ClientboundPacket for Teams {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (team_name, size) = VarString::parse(&data[offset..]).unwrap();
        offset += size;
        let (mode, size) = Byte::parse(&data[offset..]).unwrap();
        offset += size;
        let mut team_display_name = VarString("".to_string());
        let mut team_prefix = VarString("".to_string());
        let mut team_suffix = VarString("".to_string());
        let mut friendly_fire = Byte(0);
        let mut name_tag_visibility = VarString("".to_string());
        let mut color = Byte(0);
        let mut players = Vec::new();
        if mode.0 == 0 || mode.0 == 2 {
            let (_team_display_name, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            team_display_name = _team_display_name;
            let (_team_prefix, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            team_prefix = _team_prefix;
            let (_team_suffix, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            team_suffix = _team_suffix;
            let (_friendly_fire, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            friendly_fire = _friendly_fire;
            let (_name_tag_visibility, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            name_tag_visibility = _name_tag_visibility;
            let (_color, size) = Byte::parse(&data[offset..]).unwrap();
            offset += size;
            color = _color;
        }
        if mode.0 == 0 || mode.0 == 3 || mode.0 == 4 {
            let (player_count, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            for i in 0..player_count.0 {
                let (player, size) = VarString::parse(&data[offset..]).unwrap();
                offset += size;
                players.push(player);
            }
        }
        Ok(Self { team_name, mode, team_display_name, team_prefix, team_suffix, friendly_fire, name_tag_visibility, color, players })
    }
}

clientbound_packet!{
    DisconnectPlayClientbound, IDS::DisconnectPlayClientbound, {
        reason: VarString
    }
}

clientbound_packet!{
    ServerDifficulty, IDS::ServerDifficulty, {
        difficulty: UnsignedByte
    }
}

pub struct CombatEvent {
    pub event: VarInt,
    pub duration: VarInt,
    pub player_id: VarInt,
    pub entity_id: Int,
    pub message: VarString
}

impl ClientboundPacket for CombatEvent {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (event, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut duration = VarInt(0);
        let mut player_id = VarInt(0);
        let mut entity_id = Int(0);
        let mut message = VarString("".to_string());
        if event.0 == 1 {
            let (_duration, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            duration = _duration;
        }
        if event.0 == 2 {
            let (_player_id, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            player_id = _player_id;
        }
        if event.0 == 1 || event.0 == 2 {
            let (_entity_id, size) = Int::parse(&data[offset..]).unwrap();
            offset += size;
            entity_id = _entity_id;
        }
        if event.0 == 2 {
            let (_message, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            message = _message;
        }
        Ok(Self { event, duration, player_id, entity_id, message })
    }
}

clientbound_packet!{
    Camera, IDS::Camera, {
        camera_id: VarInt
    }
}

pub struct WorldBorder {
    pub action: VarInt,
    pub action_data: (Double, (Double, Double, VarLong), (Double, Double), (Double, Double, Double, Double, VarLong, VarInt, VarInt, VarInt), VarInt, VarInt)
}

impl ClientboundPacket for WorldBorder {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (action, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut x = Double(0f64);
        let mut z = Double(0f64);
        let mut radius = Double(0f64);
        let mut old_radius = Double(0f64);
        let mut new_radius = Double(0f64);
        let mut speed = VarLong(0);
        let mut portal_teleport_boundary = VarInt(0);
        let mut warning_time = VarInt(0);
        let mut warning_blocks = VarInt(0);
        if action.0 == 0 {
            let (_radius, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            radius = _radius;
        }
        if action.0 == 1 {
            let (_old_radius, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            old_radius = _old_radius;
            let (_new_radius, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            new_radius = _new_radius;
            let (_speed, size) = VarLong::parse(&data[offset..]).unwrap();
            offset += size;
            speed = _speed;
        }
        if action.0 == 2 {
            let (_x, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            x = _x;
            let (_z, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            z = _z;
        }
        if action.0 == 3 {
            let (_x, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            x = _x;
            let (_z, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            z = _z;
            let (_old_radius, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            old_radius = _old_radius;
            let (_new_radius, size) = Double::parse(&data[offset..]).unwrap();
            offset += size;
            new_radius = _new_radius;
            let (_speed, size) = VarLong::parse(&data[offset..]).unwrap();
            offset += size;
            speed = _speed;
            let (_portal_teleport_boundary, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            portal_teleport_boundary = _portal_teleport_boundary;
            let (_warning_time, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            warning_time = _warning_time;
            let (_warning_blocks, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            warning_blocks = _warning_blocks;
        }
        if action.0 == 4 {
            let (_warning_time, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            warning_time = _warning_time;
        }
        if action.0 == 5 {
            let (_warning_blocks, size) = VarInt::parse(&data[offset..]).unwrap();
            offset += size;
            warning_blocks = _warning_blocks;
        }
        let action_data = (radius, (old_radius, new_radius, speed), (x, z), (x, z, old_radius, new_radius, speed, portal_teleport_boundary, warning_time, warning_blocks), warning_time, warning_blocks);
        Ok(Self { action, action_data })
    }
}

pub struct Title {
    pub action: VarInt,
    pub title_text: VarString,
    pub subtitle_text: VarString,
    pub fade_in: Int,
    pub stay: Int,
    pub fade_out: Int,
}

impl ClientboundPacket for Title {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (action, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let mut title_text = VarString("".to_string());
        let mut subtitle_text = VarString("".to_string());
        let mut fade_in = Int(0);
        let mut stay = Int(0);
        let mut fade_out = Int(0);
        if action.0 == 0 {
            let (_title_text, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            title_text = _title_text;
        }
        if action.0 == 1 {
            let (_subtitle_text, size) = VarString::parse(&data[offset..]).unwrap();
            offset += size;
            subtitle_text = _subtitle_text;
        }
        if action.0 == 2 {
            let (_fade_in, size) = Int::parse(&data[offset..]).unwrap();
            offset += size;
            fade_in = _fade_in;
            let (_stay, size) = Int::parse(&data[offset..]).unwrap();
            offset += size;
            stay = _stay;
            let (_fade_out, size) = Int::parse(&data[offset..]).unwrap();
            offset += size;
            fade_out = _fade_out;
        }
        Ok(Self { action, title_text, subtitle_text, fade_in, stay, fade_out })
    }
}

clientbound_packet!{
    SetCompressionPlay, IDS::SetCompressionPlay, {
        threshold: VarInt
    }
}

clientbound_packet!{
    PlayerListHeaderAndFooter, IDS::PlayerListHeaderAndFooter, {
        header: VarString,
        footer: VarString
    }
}

clientbound_packet!{
    ResourcePackSend, IDS::ResourcePackSend, {
        url: VarString,
        hash: VarString
    }
}

pub struct UpdateEntityNBT {
    pub entity_id: VarInt,
    pub tag: Value
}

impl ClientboundPacket for UpdateEntityNBT {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let mut offset = 0;
        let (entity_id, size) = VarInt::parse(&data[offset..]).unwrap();
        offset += size;
        let nbt_raw = &data[offset..];
        let tag: fastnbt::Value = fastnbt::from_bytes(nbt_raw).unwrap();
        Ok(Self { entity_id, tag })
    }
}

pub trait ServerboundPacket: Sized {
    fn to_bytes(&self) -> Result<Vec<u8>, String>;
    fn id(&self) -> IDS;
}

#[macro_export]
macro_rules! serverbound_packet {
    (
        $name:ident, $id:expr, {
            $( $field_name:ident : $field_type:ty ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        pub struct $name {
            $( pub $field_name: $field_type ),*
        }

        impl $name {
            pub fn new($( $field_name: $field_type ),*) -> Self {
                Self {
                    $( $field_name ),*
                }
            }
        }

        impl ServerboundPacket for $name {
            fn to_bytes(&self) -> Result<Vec<u8>, String> {
                let mut buf = vec![];
                $(
                    self.$field_name.write(&mut buf);
                )*
                Ok(buf)
            }

            fn id(&self) -> IDS {
                $id
            }
        }
    };
}

serverbound_packet!{
    Handshake, IDS::Handshake, {
        protocol_version: VarInt,
        server_address: VarString,
        server_port: UnsignedShort,
        next_state: VarInt
    }
}

serverbound_packet!{
    LoginStart, IDS::LoginStart, {
        name: VarString
    }
}

// -- TODO Encryption Response -- //

serverbound_packet!{
    KeepAliveServerbound, IDS::KeepAliveServerbound, {
        keep_alive_id: VarInt
    }
}

serverbound_packet!{
    ChatMessageServerbound, IDS::ChatMessageServerbound, {
        message: VarString
    }
}

#[derive(Debug)]
pub struct UseEntity {
    pub  target: VarInt,
    pub _type: VarInt,
    pub target_x: Float,
    pub target_y: Float,
    pub target_z: Float
}

impl UseEntity {
    pub fn new(target: VarInt, _type: VarInt, target_x: Float, target_y: Float, target_z: Float) -> Self {
        Self {
            target,
            _type,
            target_x,
            target_y,
            target_z
        }
    }
}

impl ServerboundPacket for UseEntity {
    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut data: Vec<u8> = Vec::new();
        self.target.write(&mut data);
        self._type.write(&mut data);
        if self._type.0 == 2 {
            self.target_x.write(&mut data);
            self.target_y.write(&mut data);
            self.target_z.write(&mut data);
        }
        Ok(data)
    }

    fn id(&self) -> IDS {
        IDS::UseEntity
    }
}

serverbound_packet!{
    Player, IDS::Player, {
        on_ground: Boolean
    }
}

serverbound_packet!{
    PlayerPosition, IDS::PlayerPosition, {
        x: Double,
        feet_y: Double,
        z: Double,
        on_ground: Boolean
    }
}

serverbound_packet!{
    PlayerLook, IDS::PlayerLook, {
        yaw: Float,
        pitch: Float,
        on_ground: Boolean
    }
}

serverbound_packet!{
    PlayerPositionAndLookServerbound, IDS::PlayerPositionAndLookServerbound, {
        x: Double,
        feet_y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        on_ground: Boolean
    }
}

serverbound_packet!{
    PlayerDigging, IDS::PlayerDigging, {
        status: Byte,
        location: Position,
        face: Byte
    }
}

serverbound_packet!{
    PlayerBlockPlacement, IDS::PlayerBlockPlacement, {
        location: Position,
        face: Byte,
        held_item: Slot,
        cursor_position_x: Byte,
        cursor_position_y: Byte,
        cursot_position_z: Byte
    }
}

serverbound_packet!{
    HeldItemChangeServerbound, IDS::HeldItemChangeServerbound, {
        slot: Short
    }
}

serverbound_packet!{
    AnimationServerbound, IDS::AnimationServerbound, {}
}

serverbound_packet!{
    EntityAction, IDS::EntityAction, {
        entity_id: VarInt,
        action_id: VarInt,
        action_parameter: VarInt
    }
}

serverbound_packet!{
    SteerVehicle, IDS::SteerVehicle, {
        sideways: Float,
        forward: Float,
        flags: UnsignedByte
    }
}

serverbound_packet!{
    CloseWindowServerbound, IDS::CloseWindowServerbound, {
        window_id: UnsignedByte
    }
}

serverbound_packet!{
    ClickWindow, IDS::ClickWindow, {
        window_id: UnsignedByte,
        slot: Short,
        button: Byte,
        action_number: Short,
        mode: Byte,
        clicked_item: Slot
    }
}

serverbound_packet!{
    ConfirmTransactionServerbound, IDS::ConfirmTransactionServerbound, {
        window_id: Byte,
        action_number: Short,
        accepted: Boolean
    }
}

serverbound_packet!{
    CreativeInventoryAction, IDS::CreativeInventoryAction, {
        slot: Short,
        clicked_item: Slot
    }
}

serverbound_packet!{
    EnchantItem, IDS::EnchantItem, {
        window_id: Byte,
        enchantment: Byte
    }
}

serverbound_packet!{
    UpdateSignServerbound, IDS::UpdateSignServerbound, {
        location: Position,
        line_1: VarString,
        line_2: VarString,
        line_3: VarString,
        line_4: VarString,
    }
}

clientbound_packet!{
    PlayerAbilitiesServerbound, IDS::PlayerAbilitiesServerbound, {
        flags: Byte,
        flying_speed: Float,
        walking_speed: Float
    }
}

#[derive(Debug)]
pub struct TabCompleteServerbound {
    pub text: VarString,
    pub has_position: Boolean,
    pub looked_at_block: Position
}

impl TabCompleteServerbound {
    pub fn new(text: VarString, has_position: Boolean, looked_at_block: Position) -> Self {
        Self {
            text,
            has_position,
            looked_at_block,
        }
    }
}

impl ServerboundPacket for TabCompleteServerbound {
    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut data: Vec<u8> = Vec::new();
        self.text.write(&mut data);
        self.has_position.write(&mut data);
        if self.has_position.0 {
            self.looked_at_block.write(&mut data);
        }
        Ok(data)
    }

    fn id(&self) -> IDS {
        IDS::TabCompleteServerbound
    }
}

serverbound_packet!{
    ClientSettings, IDS::ClientSettings, {
        locale: VarString,
        view_distance: Byte,
        chat_mode: Byte,
        chat_colors: Boolean,
        displayed_skin_parts: UnsignedByte
    }
}

serverbound_packet!{
    ClientStatus, IDS::ClientStatus, {
        action_id: VarInt
    }
}

#[derive(Debug)]
pub struct PluginMessageServerbound {
    pub channel: VarString,
    pub data: Vec<u8>
}

impl PluginMessageServerbound {
    pub fn new(channel: VarString, data: Vec<u8>) -> Self {
        Self {
            channel,
            data,
        }
    }
}

impl ServerboundPacket for PluginMessageServerbound {
    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut data: Vec<u8> = Vec::new();
        self.channel.write(&mut data);
        data.extend_from_slice(self.data.as_slice());
        Ok(data)
    }

    fn id(&self) -> IDS {
        IDS::PluginMessageServerbound
    }
}

serverbound_packet!{
    Spectate, IDS::Spectate, {
        target_player: UUID
    }
}

serverbound_packet!{
    ResoucePackStatus, IDS::ResoucePackStatus, {
        hash: VarString,
        result: VarInt
    }
}