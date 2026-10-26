use std::io::{Read, Write};

use crate::wrath::{
    Map, RaidDifficulty, RaidInstanceMessage,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm:37`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm#L37):
/// ```text
/// smsg SMSG_RAID_INSTANCE_MESSAGE = 0x02FA {
///     RaidInstanceMessage message_type;
///     Map map;
///     (u32)RaidDifficulty difficulty;
///     u32 time_left;
///     if (message_type == WELCOME) {
///         u8 locked;
///         u8 extended;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage,
    pub map: Map,
    pub difficulty: RaidDifficulty,
    pub time_left: u32,
}

impl crate::private::Sealed for SMSG_RAID_INSTANCE_MESSAGE {}
impl SMSG_RAID_INSTANCE_MESSAGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=18).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // message_type: RaidInstanceMessage
        let message_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: RaidDifficulty
        let difficulty = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // time_left: u32
        let time_left = crate::util::read_u32_le(&mut r)?;

        let message_type_if = match message_type {
            RaidInstanceMessage::WarningHours => SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::WarningHours,
            RaidInstanceMessage::WarningMin => SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::WarningMin,
            RaidInstanceMessage::WarningMinSoon => SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::WarningMinSoon,
            RaidInstanceMessage::Welcome => {
                // locked: u8
                let locked = crate::util::read_u8_le(&mut r)?;

                // extended: u8
                let extended = crate::util::read_u8_le(&mut r)?;

                SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::Welcome {
                    extended,
                    locked,
                }
            }
            RaidInstanceMessage::Expired => SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::Expired,
        };

        Ok(Self {
            message_type: message_type_if,
            map,
            difficulty,
            time_left,
        })
    }

}

impl crate::Message for SMSG_RAID_INSTANCE_MESSAGE {
    const OPCODE: u32 = 0x02fa;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_RAID_INSTANCE_MESSAGE"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        w.write_all(&(self.message_type.as_int().to_le_bytes()))?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // difficulty: RaidDifficulty
        w.write_all(&u32::from(self.difficulty.as_int()).to_le_bytes())?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        match &self.message_type {
            SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::Welcome {
                extended,
                locked,
            } => {
                // locked: u8
                w.write_all(&locked.to_le_bytes())?;

                // extended: u8
                w.write_all(&extended.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(762, "SMSG_RAID_INSTANCE_MESSAGE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_RAID_INSTANCE_MESSAGE {}

impl SMSG_RAID_INSTANCE_MESSAGE {
    pub(crate) const fn size(&self) -> usize {
        self.message_type.size() // message_type: SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage
        + 4 // map: Map
        + 4 // difficulty: RaidDifficulty
        + 4 // time_left: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage {
    WarningHours,
    WarningMin,
    WarningMinSoon,
    Welcome {
        extended: u8,
        locked: u8,
    },
    Expired,
}

impl Default for SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage {
    fn default() -> Self {
        // First enumerator without any fields
        Self::WarningHours
    }
}

impl SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::WarningHours => 1,
            Self::WarningMin => 2,
            Self::WarningMinSoon => 3,
            Self::Welcome { .. } => 4,
            Self::Expired => 5,
        }
    }

}

impl std::fmt::Display for SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WarningHours => f.write_str("WarningHours"),
            Self::WarningMin => f.write_str("WarningMin"),
            Self::WarningMinSoon => f.write_str("WarningMinSoon"),
            Self::Welcome{ .. } => f.write_str("Welcome"),
            Self::Expired => f.write_str("Expired"),
        }
    }
}

impl SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Welcome {
                ..
            } => {
                4
                + 1 // extended: u8
                + 1 // locked: u8
            }
            _ => 4,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_RAID_INSTANCE_MESSAGE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 22] = [ 0x00, 0x14, 0xFA, 0x02, 0x04, 0x00, 0x00, 0x00, 0x15,
         0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x1C, 0x00, 0x00, 0x01,
         0x00, ];

    pub(crate) fn expected0() -> SMSG_RAID_INSTANCE_MESSAGE {
        SMSG_RAID_INSTANCE_MESSAGE {
            message_type: SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::Welcome {
                extended: 0x0,
                locked: 0x1,
            },
            map: Map::Naxxramas,
            difficulty: RaidDifficulty::TenManNormal,
            time_left: 0x1C20,
        }

    }

    // Generated from `wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm` line 51.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_raid_instance_message0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_RAID_INSTANCE_MESSAGE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm` line 51.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_raid_instance_message0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_RAID_INSTANCE_MESSAGE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm` line 51.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_raid_instance_message0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_RAID_INSTANCE_MESSAGE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 20] = [ 0x00, 0x12, 0xFA, 0x02, 0x01, 0x00, 0x00, 0x00, 0x15,
         0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0E, 0x00, 0x00, ];

    pub(crate) fn expected1() -> SMSG_RAID_INSTANCE_MESSAGE {
        SMSG_RAID_INSTANCE_MESSAGE {
            message_type: SMSG_RAID_INSTANCE_MESSAGE_RaidInstanceMessage::WarningHours,
            map: Map::Naxxramas,
            difficulty: RaidDifficulty::TenManNormal,
            time_left: 0xE10,
        }

    }

    // Generated from `wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm` line 71.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_raid_instance_message1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_RAID_INSTANCE_MESSAGE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm` line 71.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_raid_instance_message1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_RAID_INSTANCE_MESSAGE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm` line 71.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_raid_instance_message1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_RAID_INSTANCE_MESSAGE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

