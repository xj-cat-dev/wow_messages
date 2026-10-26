use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm#L7):
/// ```text
/// smsg SMSG_SET_FACTION_VISIBLE = 0x0123 {
///     u32 reputation_list_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SET_FACTION_VISIBLE {
    /// Index into the client's reputation list, not a faction ID.
    pub reputation_list_id: u32,
}

impl crate::private::Sealed for SMSG_SET_FACTION_VISIBLE {}
impl SMSG_SET_FACTION_VISIBLE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            reputation_list_id,
        })
    }

}

impl crate::Message for SMSG_SET_FACTION_VISIBLE {
    const OPCODE: u32 = 0x0123;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SET_FACTION_VISIBLE"
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(291, "SMSG_SET_FACTION_VISIBLE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_FACTION_VISIBLE {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SET_FACTION_VISIBLE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 8] = [ 0x00, 0x06, 0x23, 0x01, 0x05, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_SET_FACTION_VISIBLE {
        SMSG_SET_FACTION_VISIBLE {
            reputation_list_id: 0x5,
        }

    }

    // Generated from `wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm` line 14.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_set_faction_visible0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SET_FACTION_VISIBLE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SET_FACTION_VISIBLE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm` line 14.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_set_faction_visible0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SET_FACTION_VISIBLE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SET_FACTION_VISIBLE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm` line 14.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_set_faction_visible0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SET_FACTION_VISIBLE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SET_FACTION_VISIBLE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

