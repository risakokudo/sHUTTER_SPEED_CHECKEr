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
            SbeStrategy::VWAP => Self::VWAP,
            SbeStrategy::NullVal => Self::NullVal,
        }
    }
}

impl From<&Strategy> for SbeStrategy {
    #[inline]
    fn from(v: &Strategy) -> Self {
        match v {
            Strategy::SIMULATOR => Self::SIMULATOR,
            Strategy::VENUE => Self::VENUE,
            Strategy::SWEEPER => Self::SWEEPER,
            Strategy::LIMIT_SNIPER => Self::LIMIT_SNIPER,
            Strategy::STOP_LOSS => Self::STOP_LOSS,
            Strategy::TWAP => Self::TWAP,
            Strategy::VWAP => Self::VWAP,
            Strategy::NullVal => Self::NullVal,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Order {
    cl_ord_id: u64,
    cl_ord_link_id: u64,
    instrument_id: u32,
    target_comp_id: u32,
    source_comp_id: u32,
    side: OrderSide,
    order_type: OrderType,
    price: f64,
    order_qty: f64,
    time_in_force: TimeInForce,
    transact_time: u64,
    effective_time: u64,
    expire_time: u64,
    target_strategy: Strategy,
    source_strategy: Strategy,
}

impl Order {
    fn new() -> Self {
        Default::default()
    }

    fn with_cl_ord_id(mut self, cl_ord_id: u64) -> Self {
        self.cl_ord_id = cl_ord_id;
        self
    }

    fn with_cl_ord_link_id(mut self, cl_ord_link_id: u64) -> Self {
        self.cl_ord_link_id = cl_ord_link_id;
        self
    }

    fn with_instrument_id(mut self, instrument_id: u32) -> Self {
        self.instrument_id = instrument_id;
        self
    }

    fn with_target_comp_id(mut self, target_comp_id: u32) -> Self {
        self.target_comp_id = target_comp_id;
        self
    }

    fn with_source_comp_id(mut self, source_comp_id: u32) -> Self {
        self.source_comp_id = source_comp_id;
        self
    }

    fn with_side(mut self, side: OrderSide) -> Self {
        self.side = side;
        self
    }

    fn with_order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    fn with_price(mut self, price: f64) -> Self {
        self.price = price;
        self
    }

    fn with_order_qty(mut self, order_qty: f64) -> Self {
        self.order_qty = order_qty;
        self
    }

    fn with_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    fn with_transact_time