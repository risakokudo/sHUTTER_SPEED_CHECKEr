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

impl From<&OrderType> for SbeOrderType {
    #[inline]
    fn from(v: &OrderType) -> Self {
        match v {
            OrderType::MARKET => Self::MARKET,
            OrderType::LIMIT => Self::LIMIT,
            OrderType::NullVal => Self::NullVal,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
enum TimeInForce {
    DAY,
    GTC,
    IOC,
    FOK,
    GTD,
    #[default]
    NullVal,
}

impl From<&SbeTimeInForce> for TimeInForce {
    #[inline]
    fn from(v: &SbeTimeInForce) -> Self {
        match v {
            SbeTimeInForce::DAY => Self::DAY,
            SbeTimeInForce::GTC => Self::GTC,
            SbeTimeInForce::IOC => Self::IOC,
            SbeTimeInForce::FOK => Self::FOK,
            SbeTimeInForce::GTD => Self::GTD,
            SbeTimeInForce::NullVal => Self::NullVal,
        }
    }
}

impl From<&TimeInForce> for SbeTimeInForce {
    #[inline]
    fn from(v: &TimeInForce) -> Self {
        match v {
            TimeInForce::DAY => Self::DAY,
            TimeInForce::GTC => Self::GTC,
            TimeInForce::IOC => Self::IOC,
            TimeInForce::FOK => Self::FOK,
            TimeInForce::GTD => Self::GTD,
            TimeInForce::NullVal => Self::NullVal,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
#[allow(non_camel_case_types)]
enum Strategy {
    SIMULATOR,
    VENUE,
    SWEEPER,
    LIMIT_SNIPER,
    STOP_LOSS,
    TWAP,
    VWAP,
    #[default]
    NullVal,
}

impl From<&SbeStrategy> for Strategy {
    #[inline]
    fn from(v: &SbeStrategy) -> Self {
        match v {
            SbeStrategy::SIMULATOR => Self::SIMULATOR,
            SbeStrategy::VENUE => Self::VENUE,
            SbeStrategy::SWEEPER => Self::SWEEPER,
            SbeStrategy::LIMIT_SNIPER => Self::LIMIT_SNIPER,
            SbeStrategy::STOP_LOSS => Self::STOP_LOSS,
            SbeStrategy::TWAP => Self::TWAP,
            SbeStrategy::VWAP