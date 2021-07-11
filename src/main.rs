use com_dev_4fx_sor_codecs_sbe::new_order_single_codec::{SBE_BLOCK_LENGTH, SBE_SCHEMA_VERSION};
use com_dev_4fx_sor_codecs_sbe::{
    Decoder, Encoder, NewOrderSingleDecoder, NewOrderSingleEncoder, OrderSide as SbeOrderSide,
    OrderType as SbeOrderType, ReadBuf, Strategy as SbeStrategy, TimeInForce as SbeTimeInForce,
    WriteBuf,
};

#[derive(Debug, Default, PartialEq)]
enum OrderSide {
    BUY,
    SELL,
    #[default]
    NullVal,
}

impl From<&SbeOrderSide> for OrderSide {
    #[inline]
    fn from(v: &SbeOrderSide) -> Self {
        match v {
            SbeOrderSide::BUY => Self::BUY,
            SbeOrderSide::SELL => Self::SELL,
            SbeOrderSide::NullVal => Self::NullVal,
        }
    }
}

impl From<&OrderSide> for SbeOrderSide {
    #[inline]
    fn from(v: &OrderSide) -> Self {
        match v {
            OrderSide::BUY => Self::BUY,
            OrderSide::SELL => Self::SELL,
            OrderSide::NullVal => Self::NullVal,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
enum OrderType {
    MARKET,
    LIMIT,
    #[default]
    NullVal,
}

impl From<&SbeOrderType> for OrderType {
    #[inline]
    fn from(v: &SbeOrderType) -> Self {
        match v {
            SbeOrderType::MARKET => Self::MARKET,
            SbeOrderType::LIMIT => Self::LIMIT,
            SbeOrderType::NullVal => Self::NullVal,
        }
    }
}

impl From<&OrderType> for Sbe