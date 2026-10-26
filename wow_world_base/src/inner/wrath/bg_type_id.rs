/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm:59`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm#L59):
/// ```text
/// enum BgTypeId : i32 {
///     NOT_ELIGIBLE = 0;
///     QUEUED_FOR_AV = 1;
///     QUEUED_FOR_WSG = 2;
///     QUEUED_FOR_AB = 3;
///     QUEUED_FOR_NETHERSTORM = 4;
///     QUEUED_FOR_BLADES_EDGE_ARENA = 5;
///     QUEUED_FOR_ARENA = 6;
///     QUEUED_FOR_EYE_OF_THE_STORM = 7;
///     QUEUED_FOR_RUINS_OF_LORDAERON = 8;
///     QUEUED_FOR_STRAND_OF_THE_ANCIENT = 9;
///     QUEUED_FOR_DALARAN_SEWERS = 10;
///     QUEUED_FOR_RING_OF_VALOR = 11;
///     QUEUED_FOR_ISLE_OF_CONQUEST = 30;
///     ERR_BATTLEGROUND_NONE = -1;
///     ERR_GROUP_JOIN_BATTLEGROUND_DESERTERS = -2;
///     ERR_ARENA_TEAM_PARTY_SIZE = -3;
///     ERR_BATTLEGROUND_TOO_MANY_QUEUES = -4;
///     ERR_BATTLEGROUND_CANNOT_QUEUE_FOR_RATED = -5;
///     ERR_BATTLEGROUND_QUEUED_FOR_RATED = -6;
///     ERR_BATTLEGROUND_TEAM_LEFT_QUEUE = -7;
///     ERR_BATTLEGROUND_NOT_IN_BATTLEGROUND = -8;
///     ERR_BATTLEGROUND_JOIN_XP_GAIN = -9;
///     ERR_BATTLEGROUND_JOIN_RANGE_INDEX = -10;
///     ERR_BATTLEGROUND_JOIN_TIMED_OUT = -11;
///     ERR_BATTLEGROUND_JOIN_FAILED = -12;
///     ERR_LFG_CANT_USE_BATTLEGROUND = -13;
///     ERR_IN_RANDOM_BG = -14;
///     ERR_IN_NON_RANDOM_BG = -15;
/// }
/// ```
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BgTypeId {
    /// Your group has joined a battleground queue, but you are not eligible
    #[default]
    NotEligible,
    /// Your group has joined the queue for AV
    QueuedForAv,
    /// Your group has joined the queue for WS
    QueuedForWsg,
    /// Your group has joined the queue for AB
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
    /// Do not show a message.
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
    ErrBattlegroundJoinTimedOut,
    ErrBattlegroundJoinFailed,
    ErrLfgCantUseBattleground,
    ErrInRandomBg,
    ErrInNonRandomBg,
}

impl BgTypeId {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::NotEligible => 0x0,
            Self::QueuedForAv => 0x1,
            Self::QueuedForWsg => 0x2,
            Self::QueuedForAb => 0x3,
            Self::QueuedForNetherstorm => 0x4,
            Self::QueuedForBladesEdgeArena => 0x5,
            Self::QueuedForArena => 0x6,
            Self::QueuedForEyeOfTheStorm => 0x7,
            Self::QueuedForRuinsOfLordaeron => 0x8,
            Self::QueuedForStrandOfTheAncient => 0x9,
            Self::QueuedForDalaranSewers => 0xa,
            Self::QueuedForRingOfValor => 0xb,
            Self::QueuedForIsleOfConquest => 0x1e,
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
            Self::ErrBattlegroundJoinTimedOut => -11,
            Self::ErrBattlegroundJoinFailed => -12,
            Self::ErrLfgCantUseBattleground => -13,
            Self::ErrInRandomBg => -14,
            Self::ErrInNonRandomBg => -15,
        }
    }

    pub const fn variants() -> [Self; 28] {
        [
            Self::NotEligible,
            Self::QueuedForAv,
            Self::QueuedForWsg,
            Self::QueuedForAb,
            Self::QueuedForNetherstorm,
            Self::QueuedForBladesEdgeArena,
            Self::QueuedForArena,
            Self::QueuedForEyeOfTheStorm,
            Self::QueuedForRuinsOfLordaeron,
            Self::QueuedForStrandOfTheAncient,
            Self::QueuedForDalaranSewers,
            Self::QueuedForRingOfValor,
            Self::QueuedForIsleOfConquest,
            Self::ErrBattlegroundNone,
            Self::ErrGroupJoinBattlegroundDeserters,
            Self::ErrArenaTeamPartySize,
            Self::ErrBattlegroundTooManyQueues,
            Self::ErrBattlegroundCannotQueueForRated,
            Self::ErrBattlegroundQueuedForRated,
            Self::ErrBattlegroundTeamLeftQueue,
            Self::ErrBattlegroundNotInBattleground,
            Self::ErrBattlegroundJoinXpGain,
            Self::ErrBattlegroundJoinRangeIndex,
            Self::ErrBattlegroundJoinTimedOut,
            Self::ErrBattlegroundJoinFailed,
            Self::ErrLfgCantUseBattleground,
            Self::ErrInRandomBg,
            Self::ErrInNonRandomBg,
        ]
    }

    pub const fn from_int(value: i32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::NotEligible),
            1 => Ok(Self::QueuedForAv),
            2 => Ok(Self::QueuedForWsg),
            3 => Ok(Self::QueuedForAb),
            4 => Ok(Self::QueuedForNetherstorm),
            5 => Ok(Self::QueuedForBladesEdgeArena),
            6 => Ok(Self::QueuedForArena),
            7 => Ok(Self::QueuedForEyeOfTheStorm),
            8 => Ok(Self::QueuedForRuinsOfLordaeron),
            9 => Ok(Self::QueuedForStrandOfTheAncient),
            10 => Ok(Self::QueuedForDalaranSewers),
            11 => Ok(Self::QueuedForRingOfValor),
            30 => Ok(Self::QueuedForIsleOfConquest),
            -1 => Ok(Self::ErrBattlegroundNone),
            -2 => Ok(Self::ErrGroupJoinBattlegroundDeserters),
            -3 => Ok(Self::ErrArenaTeamPartySize),
            -4 => Ok(Self::ErrBattlegroundTooManyQueues),
            -5 => Ok(Self::ErrBattlegroundCannotQueueForRated),
            -6 => Ok(Self::ErrBattlegroundQueuedForRated),
            -7 => Ok(Self::ErrBattlegroundTeamLeftQueue),
            -8 => Ok(Self::ErrBattlegroundNotInBattleground),
            -9 => Ok(Self::ErrBattlegroundJoinXpGain),
            -10 => Ok(Self::ErrBattlegroundJoinRangeIndex),
            -11 => Ok(Self::ErrBattlegroundJoinTimedOut),
            -12 => Ok(Self::ErrBattlegroundJoinFailed),
            -13 => Ok(Self::ErrLfgCantUseBattleground),
            -14 => Ok(Self::ErrInRandomBg),
            -15 => Ok(Self::ErrInNonRandomBg),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl BgTypeId {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotEligible => "NOT_ELIGIBLE",
            Self::QueuedForAv => "QUEUED_FOR_AV",
            Self::QueuedForWsg => "QUEUED_FOR_WSG",
            Self::QueuedForAb => "QUEUED_FOR_AB",
            Self::QueuedForNetherstorm => "QUEUED_FOR_NETHERSTORM",
            Self::QueuedForBladesEdgeArena => "QUEUED_FOR_BLADES_EDGE_ARENA",
            Self::QueuedForArena => "QUEUED_FOR_ARENA",
            Self::QueuedForEyeOfTheStorm => "QUEUED_FOR_EYE_OF_THE_STORM",
            Self::QueuedForRuinsOfLordaeron => "QUEUED_FOR_RUINS_OF_LORDAERON",
            Self::QueuedForStrandOfTheAncient => "QUEUED_FOR_STRAND_OF_THE_ANCIENT",
            Self::QueuedForDalaranSewers => "QUEUED_FOR_DALARAN_SEWERS",
            Self::QueuedForRingOfValor => "QUEUED_FOR_RING_OF_VALOR",
            Self::QueuedForIsleOfConquest => "QUEUED_FOR_ISLE_OF_CONQUEST",
            Self::ErrBattlegroundNone => "ERR_BATTLEGROUND_NONE",
            Self::ErrGroupJoinBattlegroundDeserters => "ERR_GROUP_JOIN_BATTLEGROUND_DESERTERS",
            Self::ErrArenaTeamPartySize => "ERR_ARENA_TEAM_PARTY_SIZE",
            Self::ErrBattlegroundTooManyQueues => "ERR_BATTLEGROUND_TOO_MANY_QUEUES",
            Self::ErrBattlegroundCannotQueueForRated => "ERR_BATTLEGROUND_CANNOT_QUEUE_FOR_RATED",
            Self::ErrBattlegroundQueuedForRated => "ERR_BATTLEGROUND_QUEUED_FOR_RATED",
            Self::ErrBattlegroundTeamLeftQueue => "ERR_BATTLEGROUND_TEAM_LEFT_QUEUE",
            Self::ErrBattlegroundNotInBattleground => "ERR_BATTLEGROUND_NOT_IN_BATTLEGROUND",
            Self::ErrBattlegroundJoinXpGain => "ERR_BATTLEGROUND_JOIN_XP_GAIN",
            Self::ErrBattlegroundJoinRangeIndex => "ERR_BATTLEGROUND_JOIN_RANGE_INDEX",
            Self::ErrBattlegroundJoinTimedOut => "ERR_BATTLEGROUND_JOIN_TIMED_OUT",
            Self::ErrBattlegroundJoinFailed => "ERR_BATTLEGROUND_JOIN_FAILED",
            Self::ErrLfgCantUseBattleground => "ERR_LFG_CANT_USE_BATTLEGROUND",
            Self::ErrInRandomBg => "ERR_IN_RANDOM_BG",
            Self::ErrInNonRandomBg => "ERR_IN_NON_RANDOM_BG",
        }
    }

}

const NAME: &str = "BgTypeId";

impl std::fmt::Display for BgTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NotEligible => "NotEligible",
            Self::QueuedForAv => "QueuedForAv",
            Self::QueuedForWsg => "QueuedForWsg",
            Self::QueuedForAb => "QueuedForAb",
            Self::QueuedForNetherstorm => "QueuedForNetherstorm",
            Self::QueuedForBladesEdgeArena => "QueuedForBladesEdgeArena",
            Self::QueuedForArena => "QueuedForArena",
            Self::QueuedForEyeOfTheStorm => "QueuedForEyeOfTheStorm",
            Self::QueuedForRuinsOfLordaeron => "QueuedForRuinsOfLordaeron",
            Self::QueuedForStrandOfTheAncient => "QueuedForStrandOfTheAncient",
            Self::QueuedForDalaranSewers => "QueuedForDalaranSewers",
            Self::QueuedForRingOfValor => "QueuedForRingOfValor",
            Self::QueuedForIsleOfConquest => "QueuedForIsleOfConquest",
            Self::ErrBattlegroundNone => "ErrBattlegroundNone",
            Self::ErrGroupJoinBattlegroundDeserters => "ErrGroupJoinBattlegroundDeserters",
            Self::ErrArenaTeamPartySize => "ErrArenaTeamPartySize",
            Self::ErrBattlegroundTooManyQueues => "ErrBattlegroundTooManyQueues",
            Self::ErrBattlegroundCannotQueueForRated => "ErrBattlegroundCannotQueueForRated",
            Self::ErrBattlegroundQueuedForRated => "ErrBattlegroundQueuedForRated",
            Self::ErrBattlegroundTeamLeftQueue => "ErrBattlegroundTeamLeftQueue",
            Self::ErrBattlegroundNotInBattleground => "ErrBattlegroundNotInBattleground",
            Self::ErrBattlegroundJoinXpGain => "ErrBattlegroundJoinXpGain",
            Self::ErrBattlegroundJoinRangeIndex => "ErrBattlegroundJoinRangeIndex",
            Self::ErrBattlegroundJoinTimedOut => "ErrBattlegroundJoinTimedOut",
            Self::ErrBattlegroundJoinFailed => "ErrBattlegroundJoinFailed",
            Self::ErrLfgCantUseBattleground => "ErrLfgCantUseBattleground",
            Self::ErrInRandomBg => "ErrInRandomBg",
            Self::ErrInNonRandomBg => "ErrInNonRandomBg",
        })
    }
}

impl TryFrom<i32> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<i32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<i32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let v = i32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<u64> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<i32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<i16> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<i64> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<i32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<i32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

