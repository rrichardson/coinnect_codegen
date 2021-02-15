pub mod components {
    pub mod schemas {
        use crate::components;
        use crate::components::schemas::{
            Account, AccountBalancesItem, AggTrade, BookTicker, BookTickerList, Error, MarginOrder,
            MarginOrderDetail, MarginOrderResponseAck, MarginOrderResponseFull,
            MarginOrderResponseResult, MarginTrade, MyTrade, OcoOrder, OcoOrderOrdersItem,
            OcoOrderReport, OcoOrderReportOrdersItem, Order, OrderDetails, OrderResponseAck,
            OrderResponseFull, OrderResponseResult, PriceTicker, PriceTickerList, Ticker,
            TickerList, Trade, Transaction,
        };
        use serde_json::json;

        pub struct OcoOrderOrdersItemExample;
        impl OcoOrderOrdersItemExample {
            pub fn default() -> Option<OcoOrderOrdersItem> {
                let oco_order_orders_item: String = json!({
                    "client_order_id": Some(Some("rmBDik7yMPo1YXtIIrpDGf".to_string())),
                    "order_id": Some(Some(392119712_i64)),
                    "symbol": Some(Some("BNBUSDT".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&oco_order_orders_item).unwrap())
            }
        }

        pub struct OcoOrderExample;
        impl OcoOrderExample {
            pub fn default() -> Option<OcoOrder> {
                let oco_order: String = json!({
                    "contingency_type": Some(Some("OCO".to_string())),
                    "list_client_order_id": Some(Some("jhaxNE9gi6rItxyh8pZsoM".to_string())),
                    "list_order_status": Some(Some("ALL_DONE".to_string())),
                    "list_status_type": Some(Some("ALL_DONE".to_string())),
                    "order_list_id": Some(Some(3159531_i64)),
                    "orders": vec![OcoOrderOrdersExample::default().unwrap()],

                    "symbol": Some(Some("BNBUSDT".to_string())),
                    "transaction_time": Some(Some(1585261352335_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&oco_order).unwrap())
            }
        }

        pub struct OcoOrderReportOrdersItemExample;
        impl OcoOrderReportOrdersItemExample {
            pub fn default() -> Option<OcoOrderReportOrdersItem> {
                let oco_order_report_orders_item: String = json!({
                    "client_order_id": Some(Some("rmBDik7yMPo1YXtIIrpDGf".to_string())),
                    "order_id": Some(Some(392119712_i64)),
                    "symbol": Some(Some("BNBUSDT".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&oco_order_report_orders_item).unwrap())
            }
        }

        pub struct OcoOrderReportExample;
        impl OcoOrderReportExample {
            pub fn default() -> Option<OcoOrderReport> {
                let oco_order_report: String = json!({
                    "contingency_type": Some(Some("OCO".to_string())),
                    "list_client_order_id": Some(Some("jhaxNE9gi6rItxyh8pZsoM".to_string())),
                    "list_order_status": Some(Some("ALL_DONE".to_string())),
                    "list_status_type": Some(Some("ALL_DONE".to_string())),
                    "order_list_id": Some(Some(3159531_i64)),
                    "orders": vec![OcoOrderReportOrdersExample::default().unwrap()],

                    "symbol": Some(Some("BNBUSDT".to_string())),
                    "transaction_time": Some(Some(1585261352335_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&oco_order_report).unwrap())
            }
        }

        pub struct AccountBalancesItemExample;
        impl AccountBalancesItemExample {
            pub fn default() -> Option<AccountBalancesItem> {
                let account_balances_item: String = json!({
                    "asset": Some(Some("BTC".to_string())),
                    "free": Some(Some("4723846.89208129".to_string())),
                    "locked": Some(Some("0.00000000".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&account_balances_item).unwrap())
            }
        }

        pub struct AccountExample;
        impl AccountExample {
            pub fn default() -> Option<Account> {
                let account: String = json!({
                    "account_type": Some(Some("SPOT".to_string())),
                    "balances": vec![AccountBalancesExample::default().unwrap()],

                    "can_deposit": Some(Some(true)),
                    "can_trade": Some(Some(true)),
                    "can_withdraw": Some(Some(true)),
                    "maker_commission": Some(Some(15_i64)),
                    "taker_commission": Some(Some(15_i64)),
                    "update_time": Some(Some(123456789_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&account).unwrap())
            }
        }

        pub struct AggTradeExample;
        impl AggTradeExample {
            pub fn default() -> Option<AggTrade> {
                let agg_trade: String = json!({
                    "m": Some(Some(true)),
                    "t": Some(Some(1592892011154)),
                    "a": Some(Some(345196462_i64)),
                    "f": Some(Some(345212431_i64)),
                    "l": Some(Some(345212431_i64)),
                    "m": Some(Some(true)),
                    "p": Some(Some("9638.99000000".to_string())),
                    "q": Some(Some("0.02077200".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&agg_trade).unwrap())
            }
        }

        pub struct BookTickerExample;
        impl BookTickerExample {
            pub fn default() -> Option<BookTicker> {
                let book_ticker: String = json!({
                    "ask_price": Some(Some("16.36450000".to_string())),
                    "ask_qty": Some(Some("12.56000000".to_string())),
                    "bid_price": Some(Some("16.36240000".to_string())),
                    "bid_qty": Some(Some("256.78000000".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&book_ticker).unwrap())
            }
        }

        pub struct BookTickerListExample;
        impl BookTickerListExample {
            pub fn default() -> Option<BookTickerList> {
                None
            }
        }

        pub struct ErrorExample;
        impl ErrorExample {
            pub fn default() -> Option<Error> {
                let error: String = json!({
                    "msg": Some(Some("error message".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&error).unwrap())
            }
        }

        pub struct MarginOrderExample;
        impl MarginOrderExample {
            pub fn default() -> Option<MarginOrder> {
                let margin_order: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "cummulative_quote_qty": Some(Some("10.00000000".to_string())),
                    "executed_qty": Some(Some("10.00000000".to_string())),
                    "order_id": Some(Some(28_i64)),
                    "orig_client_order_id": Some(Some("msXkySR3u5uYwpvRMFsi3u".to_string())),
                    "orig_qty": Some(Some("10.00000000".to_string())),
                    "price": Some(Some("1.00000000".to_string())),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("CANCELED".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "type": Some(Some("LIMIT".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&margin_order).unwrap())
            }
        }

        pub struct MarginOrderDetailExample;
        impl MarginOrderDetailExample {
            pub fn default() -> Option<MarginOrderDetail> {
                let margin_order_detail: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "cummulative_quote_qty": Some(Some("10.00000000".to_string())),
                    "executed_qty": Some(Some("10.00000000".to_string())),
                    "iceberg_qty": Some(Some("0.00000000".to_string())),
                    "is_working": Some(Some(true)),
                    "order_id": Some(Some(28_i64)),
                    "orig_qty": Some(Some("10.00000000".to_string())),
                    "price": Some(Some("1.00000000".to_string())),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("CANCELED".to_string())),
                    "stop_price": Some(Some("1.00000000".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time": Some(Some(1562133008725_i64)),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "type": Some(Some("LIMIT".to_string())),
                    "update_time": Some(Some(1562133008725_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&margin_order_detail).unwrap())
            }
        }

        pub struct MarginOrderResponseAckExample;
        impl MarginOrderResponseAckExample {
            pub fn default() -> Option<MarginOrderResponseAck> {
                let margin_order_response_ack: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "order_id": Some(Some(28_i64)),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "transact_time": Some(Some(1507725176595_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&margin_order_response_ack).unwrap())
            }
        }

        pub struct MarginOrderResponseFullExample;
        impl MarginOrderResponseFullExample {
            pub fn default() -> Option<MarginOrderResponseFull> {
                let margin_order_response_full: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "cummulative_quote_qty": Some(Some(10.00000000_f32)),
                    "executed_qty": Some(Some(10.00000000_f32)),
                    "margin_buy_borrow_amount": Some(Some(5_f32)),
                    "margin_buy_borrow_asset": Some(Some("BTC".to_string())),
                    "order_id": Some(Some(28_i64)),
                    "orig_qty": Some(Some(10.00000000_f32)),
                    "price": Some(Some(1.00000000_f32)),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("FILLED".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "transact_time": Some(Some(1507725176595_i64)),
                    "type": Some(Some("MARKET".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&margin_order_response_full).unwrap())
            }
        }

        pub struct MarginOrderResponseResultExample;
        impl MarginOrderResponseResultExample {
            pub fn default() -> Option<MarginOrderResponseResult> {
                let margin_order_response_result: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "cummulative_quote_qty": Some(Some(10.00000000_f32)),
                    "executed_qty": Some(Some(10.00000000_f32)),
                    "order_id": Some(Some(28_i64)),
                    "orig_qty": Some(Some(10.00000000_f32)),
                    "price": Some(Some(1.00000000_f32)),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("FILLED".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "transact_time": Some(Some(1507725176595_i64)),
                    "type": Some(Some("MARKET".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&margin_order_response_result).unwrap())
            }
        }

        pub struct MarginTradeExample;
        impl MarginTradeExample {
            pub fn default() -> Option<MarginTrade> {
                let margin_trade: String = json!({
                    "commission": Some(Some("0.00006000".to_string())),
                    "commission_asset": Some(Some("BTC".to_string())),
                    "id": Some(Some(28_i64)),
                    "is_best_match": Some(Some(true)),
                    "is_buyer": Some(Some(true)),
                    "is_maker": Some(Some(true)),
                    "order_id": Some(Some(28_i64)),
                    "price": Some(Some("0.02000000".to_string())),
                    "qty": Some(Some("1.02000000".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time": Some(Some(1507725176595_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&margin_trade).unwrap())
            }
        }

        pub struct MyTradeExample;
        impl MyTradeExample {
            pub fn default() -> Option<MyTrade> {
                let my_trade: String = json!({
                    "commission": Some(Some("0.02077200".to_string())),
                    "commission_asset": Some(Some("BNB".to_string())),
                    "id": Some(Some(345196462_i64)),
                    "is_best_match": Some(Some(true)),
                    "is_buyer": Some(Some(true)),
                    "is_maker": Some(Some(true)),
                    "order_id": Some(Some(345196462_i64)),
                    "order_list_id": Some(Some(345196462_i64)),
                    "price": Some(Some("9638.99000000".to_string())),
                    "qty": Some(Some("0.02077200".to_string())),
                    "quote_qty": Some(Some("0.02077200".to_string())),
                    "symbol": Some(Some("BNBUSDT".to_string())),
                    "time": Some(Some(1592887772684_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&my_trade).unwrap())
            }
        }

        pub struct OrderExample;
        impl OrderExample {
            pub fn default() -> Option<Order> {
                let order: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "cummulative_quote_qty": Some(Some(10.00000000_f32)),
                    "executed_qty": Some(Some(10.00000000_f32)),
                    "order_id": Some(Some(28_i64)),
                    "order_list_id": Some(Some(-1_i64)),
                    "orig_client_order_id": Some(Some("msXkySR3u5uYwpvRMFsi3u".to_string())),
                    "orig_qty": Some(Some(10.00000000_f32)),
                    "price": Some(Some(1.00000000_f32)),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("FILLED".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "type": Some(Some("LIMIT".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&order).unwrap())
            }
        }

        pub struct OrderDetailsExample;
        impl OrderDetailsExample {
            pub fn default() -> Option<OrderDetails> {
                let order_details: String = json!({
                    "cummulative_quote_qty": Some(Some(10.00000000_f32)),
                    "executed_qty": Some(Some(10.00000000_f32)),
                    "iceberg_qty": Some(Some(0.00000000_f32)),
                    "is_working": Some(Some(true)),
                    "order_id": Some(Some(28_i64)),
                    "order_list_id": Some(Some(-1_i64)),
                    "orig_client_order_id": Some(Some("msXkySR3u5uYwpvRMFsi3u".to_string())),
                    "orig_qty": Some(Some(10.00000000_f32)),
                    "orig_quote_order_qty": Some(Some(0.00000000_f32)),
                    "price": Some(Some(1.00000000_f32)),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("FILLED".to_string())),
                    "stop_price": Some(Some(10.00000000_f32)),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time": Some(Some(1507725176595_i64)),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "type": Some(Some("LIMIT".to_string())),
                    "update_time": Some(Some(1507725176595_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&order_details).unwrap())
            }
        }

        pub struct OrderResponseAckExample;
        impl OrderResponseAckExample {
            pub fn default() -> Option<OrderResponseAck> {
                let order_response_ack: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "order_id": Some(Some(28_i64)),
                    "order_list_id": Some(Some(-1_i64)),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "transact_time": Some(Some(1507725176595_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&order_response_ack).unwrap())
            }
        }

        pub struct OrderResponseFullExample;
        impl OrderResponseFullExample {
            pub fn default() -> Option<OrderResponseFull> {
                let order_response_full: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "cummulative_quote_qty": Some(Some(10.00000000_f32)),
                    "executed_qty": Some(Some(10.00000000_f32)),
                    "order_id": Some(Some(28_i64)),
                    "order_list_id": Some(Some(-1_i64)),
                    "orig_qty": Some(Some(10.00000000_f32)),
                    "price": Some(Some(1.00000000_f32)),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("FILLED".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "transact_time": Some(Some(1507725176595_i64)),
                    "type": Some(Some("MARKET".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&order_response_full).unwrap())
            }
        }

        pub struct OrderResponseResultExample;
        impl OrderResponseResultExample {
            pub fn default() -> Option<OrderResponseResult> {
                let order_response_result: String = json!({
                    "client_order_id": Some(Some("6gCrw2kRUAF9CvJDGP16IP".to_string())),
                    "cummulative_quote_qty": Some(Some(10.00000000_f32)),
                    "executed_qty": Some(Some(10.00000000_f32)),
                    "order_id": Some(Some(28_i64)),
                    "order_list_id": Some(Some(-1_i64)),
                    "orig_qty": Some(Some(10.00000000_f32)),
                    "price": Some(Some(1.00000000_f32)),
                    "side": Some(Some("SELL".to_string())),
                    "status": Some(Some("FILLED".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "time_in_force": Some(Some("GTC".to_string())),
                    "transact_time": Some(Some(1507725176595_i64)),
                    "type": Some(Some("MARKET".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&order_response_result).unwrap())
            }
        }

        pub struct PriceTickerExample;
        impl PriceTickerExample {
            pub fn default() -> Option<PriceTicker> {
                let price_ticker: String = json!({
                    "price": Some(Some("0.17160000".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&price_ticker).unwrap())
            }
        }

        pub struct PriceTickerListExample;
        impl PriceTickerListExample {
            pub fn default() -> Option<PriceTickerList> {
                None
            }
        }

        pub struct TickerExample;
        impl TickerExample {
            pub fn default() -> Option<Ticker> {
                let ticker: String = json!({
                    "ask_price": Some(Some("16.35920000".to_string())),
                    "ask_qty": Some(Some("25.06000000".to_string())),
                    "bid_price": Some(Some("16.34488284".to_string())),
                    "bid_qty": Some(Some("16.34488284".to_string())),
                    "close_time": Some(Some(1592895188637_i64)),
                    "count": Some(Some(55958_i64)),
                    "first_id": Some(Some(62683296_i64)),
                    "high_price": Some(Some("16.55000000".to_string())),
                    "last_id": Some(Some(62739253_i64)),
                    "last_price": Some(Some("27.84000000".to_string())),
                    "low_price": Some(Some("16.16940000".to_string())),
                    "open_price": Some(Some("16.18760000".to_string())),
                    "open_time": Some(Some(1592808788637_i64)),
                    "prev_close_price": Some(Some("16.35920000".to_string())),
                    "price_change": Some(Some("0.17160000".to_string())),
                    "price_change_percent": Some(Some("1.060".to_string())),
                    "quote_volume": Some(Some("27431289.14792300".to_string())),
                    "symbol": Some(Some("BNBBTC".to_string())),
                    "volume": Some(Some("1678279.95000000".to_string())),
                })
                .to_string();
                Some(serde_json::from_str(&ticker).unwrap())
            }
        }

        pub struct TickerListExample;
        impl TickerListExample {
            pub fn default() -> Option<TickerList> {
                None
            }
        }

        pub struct TradeExample;
        impl TradeExample {
            pub fn default() -> Option<Trade> {
                let trade: String = json!({
                    "id": Some(Some(345196462_i64)),
                    "is_best_match": Some(Some(true)),
                    "is_buyer_maker": Some(Some(true)),
                    "price": Some(Some("9638.99000000".to_string())),
                    "qty": Some(Some("0.02077200".to_string())),
                    "quote_qty": Some(Some("0.02077200".to_string())),
                    "time": Some(Some(1592887772684_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&trade).unwrap())
            }
        }

        pub struct TransactionExample;
        impl TransactionExample {
            pub fn default() -> Option<Transaction> {
                let transaction: String = json!({
                    "tran_id": Some(Some(345196462_i64)),
                })
                .to_string();
                Some(serde_json::from_str(&transaction).unwrap())
            }
        }
    }
}
