use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::BgTypeId;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm:48`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm#L48):
/// ```text
/// smsg SMSG_GROUP_JOINED_BATTLEGROUND = 0x02E8 {
///     BgTypeId id;
///     if (id == ERR_BATTLEGROUND_JOIN_TIMED_OUT
///         || id == ERR_BATTLEGROUND_JOIN_FAILED) {
///         Guid player;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_GROUP_JOINED_BATTLEGROUND {
    NotEligible,
    QueuedForAv,
    QueuedForWsg,
    QueuedForAb,
    QueuedForNetherstorm,
    QueuedForBladesEdgeArena,
    QueuedForArena,
    QueuedForEyeOfTheStorm,
    QueuedForRuinsOfLordaeron,
    QueuedForStrandOfTheAncient,
    QueuedForDalaranSewers,
    QueuedForRingOfValor,
    QueuedForIsleOfConquest,
    ErrBattlegroundNone,
    ErrGroupJoinBattlegroundDeserters,
    ErrArenaTeamPartySize,
    ErrBattlegroundTooManyQueues,
    ErrBattlegroundCannotQueueForRated,
    ErrBattlegroundQueuedForRated,
    ErrBattlegroundTeamLeftQueue,
    ErrBattlegroundNotInBattleground,
    ErrBattlegroundJoinXpGain,
    ErrBattlegroundJoinRangeIndex,
    ErrBattlegroundJoinTimedOut {
        player: Guid,
    },
    ErrBattlegroundJoinFailed {
        player: Guid,
    },
    ErrLfgCantUseBattleground,
    ErrInRandomBg,
    ErrInNonRandomBg,
}

impl crate::private::Sealed for SMSG_GROUP_JOINED_BATTLEGROUND {}
impl SMSG_GROUP_JOINED_BATTLEGROUND {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=12).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // id: BgTypeId
        let id = crate::util::read_i32_le(&mut r)?.try_into()?;

        let id_if = match id {
            BgTypeId::NotEligible => SMSG_GROUP_JOINED_BATTLEGROUND::NotEligible,
            BgTypeId::QueuedForAv => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForAv,
            BgTypeId::QueuedForWsg => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForWsg,
            BgTypeId::QueuedForAb => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForAb,
            BgTypeId::QueuedForNetherstorm => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForNetherstorm,
            BgTypeId::QueuedForBladesEdgeArena => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForBladesEdgeArena,
            BgTypeId::QueuedForArena => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForArena,
            BgTypeId::QueuedForEyeOfTheStorm => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForEyeOfTheStorm,
            BgTypeId::QueuedForRuinsOfLordaeron => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForRuinsOfLordaeron,
            BgTypeId::QueuedForStrandOfTheAncient => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForStrandOfTheAncient,
            BgTypeId::QueuedForDalaranSewers => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForDalaranSewers,
            BgTypeId::QueuedForRingOfValor => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForRingOfValor,
            BgTypeId::QueuedForIsleOfConquest => SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForIsleOfConquest,
            BgTypeId::ErrBattlegroundNone => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundNone,
            BgTypeId::ErrGroupJoinBattlegroundDeserters => SMSG_GROUP_JOINED_BATTLEGROUND::ErrGroupJoinBattlegroundDeserters,
            BgTypeId::ErrArenaTeamPartySize => SMSG_GROUP_JOINED_BATTLEGROUND::ErrArenaTeamPartySize,
            BgTypeId::ErrBattlegroundTooManyQueues => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundTooManyQueues,
            BgTypeId::ErrBattlegroundCannotQueueForRated => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundCannotQueueForRated,
            BgTypeId::ErrBattlegroundQueuedForRated => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundQueuedForRated,
            BgTypeId::ErrBattlegroundTeamLeftQueue => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundTeamLeftQueue,
            BgTypeId::ErrBattlegroundNotInBattleground => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundNotInBattleground,
            BgTypeId::ErrBattlegroundJoinXpGain => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinXpGain,
            BgTypeId::ErrBattlegroundJoinRangeIndex => SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinRangeIndex,
            BgTypeId::ErrBattlegroundJoinTimedOut => {
                // player: Guid
                let player = crate::util::read_guid(&mut r)?;

                SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinTimedOut {
                    player,
                }
            }
            BgTypeId::ErrBattlegroundJoinFailed => {
                // player: Guid
                let player = crate::util::read_guid(&mut r)?;

                SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinFailed {
                    player,
                }
            }
            BgTypeId::ErrLfgCantUseBattleground => SMSG_GROUP_JOINED_BATTLEGROUND::ErrLfgCantUseBattleground,
            BgTypeId::ErrInRandomBg => SMSG_GROUP_JOINED_BATTLEGROUND::ErrInRandomBg,
            BgTypeId::ErrInNonRandomBg => SMSG_GROUP_JOINED_BATTLEGROUND::ErrInNonRandomBg,
        };

        Ok(id_if)
    }

}

impl crate::Message for SMSG_GROUP_JOINED_BATTLEGROUND {
    const OPCODE: u32 = 0x02e8;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_GROUP_JOINED_BATTLEGROUND"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: BgTypeId
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinTimedOut {
                player,
            } => {
                // player: Guid
                w.write_all(&player.guid().to_le_bytes())?;

            }
            SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinFailed {
                player,
            } => {
                // player: Guid
                w.write_all(&player.guid().to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(744, "SMSG_GROUP_JOINED_BATTLEGROUND", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GROUP_JOINED_BATTLEGROUND {}

impl SMSG_GROUP_JOINED_BATTLEGROUND {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::ErrBattlegroundJoinTimedOut {
                ..
            } => {
                4
                + 8 // player: Guid
            }
            Self::ErrBattlegroundJoinFailed {
                ..
            } => {
                4
                + 8 // player: Guid
            }
            _ => 4,
        }) // id: SMSG_GROUP_JOINED_BATTLEGROUND
    }
}

impl Default for SMSG_GROUP_JOINED_BATTLEGROUND {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotEligible
    }
}

impl SMSG_GROUP_JOINED_BATTLEGROUND {
    pub(crate) const fn as_int(&self) -> i32 {
        match self {
            Self::NotEligible => 0,
            Self::QueuedForAv => 1,
            Self::QueuedForWsg => 2,
            Self::QueuedForAb => 3,
            Self::QueuedForNetherstorm => 4,
            Self::QueuedForBladesEdgeArena => 5,
            Self::QueuedForArena => 6,
            Self::QueuedForEyeOfTheStorm => 7,
            Self::QueuedForRuinsOfLordaeron => 8,
            Self::QueuedForStrandOfTheAncient => 9,
            Self::QueuedForDalaranSewers => 10,
            Self::QueuedForRingOfValor => 11,
            Self::QueuedForIsleOfConquest => 30,
            Self::ErrBattlegroundNone => -1,
            Self::ErrGroupJoinBattlegroundDeserters => -2,
            Self::ErrArenaTeamPartySize => -3,
            Self::ErrBattlegroundTooManyQueues => -4,
            Self::ErrBattlegroundCannotQueueForRated => -5,
            Self::ErrBattlegroundQueuedForRated => -6,
            Self::ErrBattlegroundTeamLeftQueue => -7,
            Self::ErrBattlegroundNotInBattleground => -8,
            Self::ErrBattlegroundJoinXpGain => -9,
            Self::ErrBattlegroundJoinRangeIndex => -10,
            Self::ErrBattlegroundJoinTimedOut { .. } => -11,
            Self::ErrBattlegroundJoinFailed { .. } => -12,
            Self::ErrLfgCantUseBattleground => -13,
            Self::ErrInRandomBg => -14,
            Self::ErrInNonRandomBg => -15,
        }
    }

}

impl std::fmt::Display for SMSG_GROUP_JOINED_BATTLEGROUND {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEligible => f.write_str("NotEligible"),
            Self::QueuedForAv => f.write_str("QueuedForAv"),
            Self::QueuedForWsg => f.write_str("QueuedForWsg"),
            Self::QueuedForAb => f.write_str("QueuedForAb"),
            Self::QueuedForNetherstorm => f.write_str("QueuedForNetherstorm"),
            Self::QueuedForBladesEdgeArena => f.write_str("QueuedForBladesEdgeArena"),
            Self::QueuedForArena => f.write_str("QueuedForArena"),
            Self::QueuedForEyeOfTheStorm => f.write_str("QueuedForEyeOfTheStorm"),
            Self::QueuedForRuinsOfLordaeron => f.write_str("QueuedForRuinsOfLordaeron"),
            Self::QueuedForStrandOfTheAncient => f.write_str("QueuedForStrandOfTheAncient"),
            Self::QueuedForDalaranSewers => f.write_str("QueuedForDalaranSewers"),
            Self::QueuedForRingOfValor => f.write_str("QueuedForRingOfValor"),
            Self::QueuedForIsleOfConquest => f.write_str("QueuedForIsleOfConquest"),
            Self::ErrBattlegroundNone => f.write_str("ErrBattlegroundNone"),
            Self::ErrGroupJoinBattlegroundDeserters => f.write_str("ErrGroupJoinBattlegroundDeserters"),
            Self::ErrArenaTeamPartySize => f.write_str("ErrArenaTeamPartySize"),
            Self::ErrBattlegroundTooManyQueues => f.write_str("ErrBattlegroundTooManyQueues"),
            Self::ErrBattlegroundCannotQueueForRated => f.write_str("ErrBattlegroundCannotQueueForRated"),
            Self::ErrBattlegroundQueuedForRated => f.write_str("ErrBattlegroundQueuedForRated"),
            Self::ErrBattlegroundTeamLeftQueue => f.write_str("ErrBattlegroundTeamLeftQueue"),
            Self::ErrBattlegroundNotInBattleground => f.write_str("ErrBattlegroundNotInBattleground"),
            Self::ErrBattlegroundJoinXpGain => f.write_str("ErrBattlegroundJoinXpGain"),
            Self::ErrBattlegroundJoinRangeIndex => f.write_str("ErrBattlegroundJoinRangeIndex"),
            Self::ErrBattlegroundJoinTimedOut{ .. } => f.write_str("ErrBattlegroundJoinTimedOut"),
            Self::ErrBattlegroundJoinFailed{ .. } => f.write_str("ErrBattlegroundJoinFailed"),
            Self::ErrLfgCantUseBattleground => f.write_str("ErrLfgCantUseBattleground"),
            Self::ErrInRandomBg => f.write_str("ErrInRandomBg"),
            Self::ErrInNonRandomBg => f.write_str("ErrInNonRandomBg"),
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_GROUP_JOINED_BATTLEGROUND;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xE8, 0x02, 0x01, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_GROUP_JOINED_BATTLEGROUND {
        SMSG_GROUP_JOINED_BATTLEGROUND::QueuedForAv
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 97.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_group_joined_battleground0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 97.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_group_joined_battleground0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 97.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_group_joined_battleground0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 8] = [ 0x00, 0x06, 0xE8, 0x02, 0xF1, 0xFF, 0xFF, 0xFF, ];

    pub(crate) fn expected1() -> SMSG_GROUP_JOINED_BATTLEGROUND {
        SMSG_GROUP_JOINED_BATTLEGROUND::ErrInNonRandomBg
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 107.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_group_joined_battleground1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 107.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_group_joined_battleground1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 107.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_group_joined_battleground1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 16] = [ 0x00, 0x0E, 0xE8, 0x02, 0xF5, 0xFF, 0xFF, 0xFF, 0x08,
         0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, ];

    pub(crate) fn expected2() -> SMSG_GROUP_JOINED_BATTLEGROUND {
        SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinTimedOut {
            player: Guid::new(0x102030405060708),
        }
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 117.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_group_joined_battleground2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 117.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_group_joined_battleground2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 117.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_group_joined_battleground2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 16] = [ 0x00, 0x0E, 0xE8, 0x02, 0xF4, 0xFF, 0xFF, 0xFF, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected3() -> SMSG_GROUP_JOINED_BATTLEGROUND {
        SMSG_GROUP_JOINED_BATTLEGROUND::ErrBattlegroundJoinFailed {
            player: Guid::new(0x0),
        }
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 129.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_group_joined_battleground3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 129.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_group_joined_battleground3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm` line 129.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_group_joined_battleground3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_GROUP_JOINED_BATTLEGROUND, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

}

