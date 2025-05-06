use rust2go::R2G;
use hftbacktest::types::{OrdType,OrderId,Side,Status,TimeInForce};

#[repr(C, align(64))]
#[derive(R2G,Clone, PartialEq, Debug)]//, NpyDTyped, Decode, Encode)]
pub struct Event {
    /// Event flag
    pub ev: u64,
    /// Exchange timestamp, which is the time at which the event occurs on the exchange.
    pub exch_ts: i64,
    /// Local timestamp, which is the time at which the event is received by the local.
    pub local_ts: i64,
    /// Price
    pub px: f64,
    /// Quantity
    pub qty: f64,
    /// Order ID is only for the L3 Market-By-Order feed.
    pub order_id: u64,
    /// Reserved for an additional i64 value
    pub ival: i64,
    /// Reserved for an additional f64 value
    pub fval: f64,
}

/// Represents a side, which can refer to either the side of an order or the initiator's side in a
#[derive(R2G,Clone,Debug)]
#[repr(C)]
pub struct Order {
    /// Order quantity
    pub qty: f64,
    /// The quantity of this order that has not yet been executed. It represents the remaining
    /// quantity that is still open or active in the market after any partial fills.
    pub leaves_qty: f64,
    /// Executed quantity, only available when this order is executed.
    pub exec_qty: f64,
    /// Executed price in ticks (`executed_price / tick_size`), only available when this order is
    /// executed.
    pub exec_price_tick: i64,
    /// Order price in ticks (`price / tick_size`).
    pub price_tick: i64,
    /// The tick size of the asset associated with this order.
    pub tick_size: f64,
    /// The time at which the exchange processes this order, ideally when the matching engine
    /// processes the order, will be set if the value is available.
    pub exch_timestamp: i64,
    /// The time at which the local receives this order or sent this order to the exchange.
    pub local_timestamp: i64,
    pub order_id: u64,
    /// Additional data used for [`QueueModel`](`crate::backtest::models::QueueModel`).
    /// This is only available in backtesting, and the type `Q` is set to `()` in a live bot.
    // pub q:  MaybeUninit<Box<dyn AnyClone + Send>>,
    /// Whether the order is executed as a maker, only available when this order is executed.
    pub maker: bool,
    pub order_type: u8,
    /// Request status:
    ///   * [`Status::New`]: Request to open a new order.
    ///   * [`Status::Canceled`]: Request to cancel an opened order.
    pub req: u8,
    pub status: u8,
    pub side: i8,
    pub time_in_force: u8
}

impl TryFrom<Order> for hftbacktest::types::Order {
    type Error = &'static str;
    fn try_from(a: Order) -> Self {
        let ordt = OrdType::try_from(a.order_type).expect("order:OrdType try_from failed");
        let stat = Status::try_from(a.status).expect("statue: Status try_from failedd");    
        let req = Status::try_from(a.req).expect("req: Status try_from failedd");
        let side = Side::try_from(a.side).expect("side: Side try_from failedd");
        let timeforce = TimeInForce::try_from(a.time_in_force).expect("time_in_force: TimeInForce try_from failedd");
        hftbacktest::types::Order {q:Box::new(()),qty: a.qty, leaves_qty: a.leaves_qty, exec_qty: a.exec_qty, exec_price_tick: a.exec_price_tick, price_tick: a.price_tick, tick_size: a.tick_size, exch_timestamp: a.exch_timestamp, local_timestamp: a.local_timestamp, order_id:a.order_id , maker: a.maker, order_type: ord, req:req, status: stat, side: side, time_in_force: timeforce }
        
    }
}