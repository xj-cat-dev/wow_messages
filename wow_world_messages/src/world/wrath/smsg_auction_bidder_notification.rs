use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AuctionHouse;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm#L39):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_NOTIFICATION = 0x025E {
///     AuctionHouse auction_house;
///     u32 auction_id;
///     Guid bidder;
///     u32 bid_sum;
///     u32 minimum_increment;
///     u32 item_template;
///     u32 item_random_property_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub auction_house: AuctionHouse,
    pub auction_id: u32,
    pub bidder: Guid,
    pub bid_sum: u32,
    pub minimum_increment: u32,
    pub item_template: u32,
    pub item_random_property_id: u32,
}

impl crate::private::Sealed for SMSG_AUCTION_BIDDER_NOTIFICATION {}
impl SMSG_AUCTION_BIDDER_NOTIFICATION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 32 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // auction_house: AuctionHouse
        let auction_house = crate::util::read_u32_le(&mut r)?.try_into()?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // bidder: Guid
        let bidder = crate::util::read_guid(&mut r)?;

        // bid_sum: u32
        let bid_sum = crate::util::read_u32_le(&mut r)?;

        // minimum_increment: u32
        let minimum_increment = crate::util::read_u32_le(&mut r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auction_house,
            auction_id,
            bidder,
            bid_sum,
            minimum_increment,
            item_template,
            item_random_property_id,
        })
    }

}

impl crate::Message for SMSG_AUCTION_BIDDER_NOTIFICATION {
    const OPCODE: u32 = 0x025e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_AUCTION_BIDDER_NOTIFICATION"
    }

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auction_house: AuctionHouse
        w.write_all(&(self.auction_house.as_int().to_le_bytes()))?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

        // bid_sum: u32
        w.write_all(&self.bid_sum.to_le_bytes())?;

        // minimum_increment: u32
        w.write_all(&self.minimum_increment.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(606, "SMSG_AUCTION_BIDDER_NOTIFICATION", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUCTION_BIDDER_NOTIFICATION {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_AUCTION_BIDDER_NOTIFICATION;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 36] = [ 0x00, 0x22, 0x5E, 0x02, 0x01, 0x00, 0x00, 0x00, 0x2A,
         0x00, 0x00, 0x00, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10,
         0x27, 0x00, 0x00, 0xF4, 0x01, 0x00, 0x00, 0x24, 0x1B, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_AUCTION_BIDDER_NOTIFICATION {
        SMSG_AUCTION_BIDDER_NOTIFICATION {
            auction_house: AuctionHouse::Stormwind,
            auction_id: 0x2A,
            bidder: Guid::new(0x102030405060708),
            bid_sum: 0x2710,
            minimum_increment: 0x1F4,
            item_template: 0x1B24,
            item_random_property_id: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm` line 51.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_auction_bidder_notification0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUCTION_BIDDER_NOTIFICATION, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(32 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm` line 51.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_auction_bidder_notification0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUCTION_BIDDER_NOTIFICATION, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(32 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm` line 51.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_auction_bidder_notification0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUCTION_BIDDER_NOTIFICATION, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(32 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

