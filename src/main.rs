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
    fn