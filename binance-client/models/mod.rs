#![allow(clippy::clone_on_copy)]

pub mod components {
    pub mod schemas {
        use super::super::components;
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use std::convert::TryFrom;
        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OcoOrderOrdersItem {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OcoOrder {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub contingency_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub list_client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub list_order_status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub list_status_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orders: Option<Vec<OcoOrderOrdersItem>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transaction_time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OcoOrderReportOrdersItem {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OcoOrderReport {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub contingency_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub list_client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub list_order_status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub list_status_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_reports: Option<Vec<components::schemas::Order>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orders: Option<Vec<OcoOrderReportOrdersItem>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transaction_time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct AccountBalancesItem {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub asset: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub free: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub locked: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Account {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub account_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub balances: Option<Vec<AccountBalancesItem>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub buyer_commission: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub can_deposit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub can_trade: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub can_withdraw: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maker_commission: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub seller_commission: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub taker_commission: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub update_time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct AggTrade {
            #[serde(rename = "m")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub m_: Option<bool>,
            #[serde(rename = "t")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub t_: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub a: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub f: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub l: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub m: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub p: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub q: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct BookTicker {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ask_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ask_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub bid_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub bid_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
        }

        pub type BookTickerList = Vec<components::schemas::BookTicker>;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Error {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub code: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub msg: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct MarginOrder {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct MarginOrderDetail {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub iceberg_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_working: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stop_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub update_time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct MarginOrderResponseAck {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transact_time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct MarginOrderResponseFull {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fills: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub margin_buy_borrow_amount: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub margin_buy_borrow_asset: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transact_time: Option<i64>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct MarginOrderResponseResult {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transact_time: Option<i64>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct MarginTrade {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub commission: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub commission_asset: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_best_match: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_buyer: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_maker: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct MyTrade {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub commission: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub commission_asset: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_best_match: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_buyer: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_maker: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub quote_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Order {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OrderDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub iceberg_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_working: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_quote_order_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stop_price: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub update_time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OrderResponseAck {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transact_time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OrderResponseFull {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fills: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transact_time: Option<i64>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct OrderResponseResult {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_order_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cummulative_quote_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub executed_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub order_list_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub orig_qty: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub side: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_in_force: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transact_time: Option<i64>,
            #[serde(rename = "type")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub type_: Option<String>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct PriceTicker {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
        }

        pub type PriceTickerList = Vec<components::schemas::PriceTicker>;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Ticker {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ask_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ask_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub bid_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub bid_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub close_time: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub count: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub first_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub high_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub last_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub last_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub low_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub open_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub open_time: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub prev_close_price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price_change: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price_change_percent: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub quote_volume: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub volume: Option<String>,
        }

        pub type TickerList = Vec<components::schemas::Ticker>;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Trade {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_best_match: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_buyer_maker: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub price: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub quote_qty: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time: Option<i64>,
        }

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Transaction {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tran_id: Option<i64>,
        }
    }
}

pub mod get_api_v3_account {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_account operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_account operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  account details
    pub type Response200 = components::schemas::Account;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  account details
    pub type Status200 = components::schemas::Account;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_agg_trades {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_agg_trades operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                from_id: query.from_id,
                start_time: query.start_time,
                end_time: query.end_time,
                limit: query.limit,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                from_id: self.from_id.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                limit: self.limit.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  Trade id to fetch from. Default gets most recent trades.
    pub type FromId = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    /// Query parameters for get_api_v3_agg_trades operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(rename = "from_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  trade list
    pub type Response200 = Vec<components::schemas::AggTrade>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  trade list
    pub type Status200 = Vec<components::schemas::AggTrade>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_api_v3_all_order_list {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_all_order_list operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                from_id: query.from_id,
                start_time: query.start_time,
                end_time: query.end_time,
                limit: query.limit,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                from_id: self.from_id.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                limit: self.limit.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  Trade id to fetch from. Default gets most recent trades.
    pub type FromId = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_all_order_list operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(rename = "from_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  oco order list
    pub type Response200 = Vec<components::schemas::OcoOrder>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  oco order list
    pub type Status200 = Vec<components::schemas::OcoOrder>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_all_orders {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_all_orders operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  order id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                order_id: query.order_id,
                start_time: query.start_time,
                end_time: query.end_time,
                limit: query.limit,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                order_id: self.order_id.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                limit: self.limit.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  order id
    pub type OrderId = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_all_orders operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  order id
        #[serde(rename = "order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  current open orders
    pub type Response200 = Vec<components::schemas::OrderDetails>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  current open orders
    pub type Status200 = Vec<components::schemas::OrderDetails>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_avg_price {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_avg_price operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    /// Query parameters for get_api_v3_avg_price operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  average price
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mins: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<String>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  average price
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mins: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<String>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_api_v3_depth {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_depth operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                limit: query.limit,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                limit: self.limit.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    pub type Limit = i64;

    /// Query parameters for get_api_v3_depth operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,

        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  order books
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asks: Option<Vec<Vec<Vec<String>>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bids: Option<Vec<Vec<Vec<String>>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_update_id: Option<i64>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  order books
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asks: Option<Vec<Vec<Vec<String>>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bids: Option<Vec<Vec<Vec<String>>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_update_id: Option<i64>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_api_v3_exchange_info {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_exchange_info operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    pub type Response200ExchangeFiltersItem = ();

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200RateLimitsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interval_num: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rate_limit_type: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200SymbolsItemFiltersItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filter_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tick_size: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200SymbolsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base_asset_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base_commission_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filters: Option<Vec<Response200SymbolsItemFiltersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_margin_trading_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_spot_trading_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub oco_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permissions: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_asset_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_commission_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_order_qty_market_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    ///  Current exchange trading rules and symbol information
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exchange_filters: Option<Vec<Response200ExchangeFiltersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rate_limits: Option<Vec<Response200RateLimitsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub server_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbols: Option<Vec<Response200SymbolsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timezone: Option<String>,
    }

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    pub type Status200ExchangeFiltersItem = ();

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200RateLimitsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interval: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interval_num: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rate_limit_type: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200SymbolsItemFiltersItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filter_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tick_size: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200SymbolsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base_asset_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base_commission_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filters: Option<Vec<Status200SymbolsItemFiltersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_margin_trading_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_spot_trading_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub oco_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_types: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permissions: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_asset_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_commission_precision: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_order_qty_market_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    ///  Current exchange trading rules and symbol information
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exchange_filters: Option<Vec<Status200ExchangeFiltersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rate_limits: Option<Vec<Status200RateLimitsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub server_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbols: Option<Vec<Status200SymbolsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timezone: Option<String>,
    }
}

pub mod get_api_v3_historical_trades {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_historical_trades operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                limit: query.limit,
                from_id: query.from_id,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                limit: self.limit.clone(),
                from_id: self.from_id.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  Trade id to fetch from. Default gets most recent trades.
    pub type FromId = i64;

    /// Query parameters for get_api_v3_historical_trades operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(rename = "from_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  trade list
    pub type Response200 = Vec<components::schemas::Trade>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  trade list
    pub type Status200 = Vec<components::schemas::Trade>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_klines {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_klines operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  kline intervals
        pub interval: Interval,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                interval: query.interval,
                start_time: query.start_time,
                end_time: query.end_time,
                limit: query.limit,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                interval: self.interval.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                limit: self.limit.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  kline intervals

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Interval {
        #[serde(rename = "1m")]
        #[allow(non_camel_case_types)]
        Onem,
        #[serde(rename = "3m")]
        #[allow(non_camel_case_types)]
        Threem,
        #[serde(rename = "5m")]
        #[allow(non_camel_case_types)]
        Fivem,
        #[serde(rename = "15m")]
        #[allow(non_camel_case_types)]
        Fifteenm,
        #[serde(rename = "30m")]
        #[allow(non_camel_case_types)]
        Thirtym,
        #[serde(rename = "1h")]
        #[allow(non_camel_case_types)]
        Oneh,
        #[serde(rename = "2h")]
        #[allow(non_camel_case_types)]
        Twoh,
        #[serde(rename = "4h")]
        #[allow(non_camel_case_types)]
        Fourh,
        #[serde(rename = "6h")]
        #[allow(non_camel_case_types)]
        Sixh,
        #[serde(rename = "8h")]
        #[allow(non_camel_case_types)]
        Eighth,
        #[serde(rename = "12h")]
        #[allow(non_camel_case_types)]
        Twelveh,
        #[serde(rename = "1d")]
        #[allow(non_camel_case_types)]
        Oned,
        #[serde(rename = "3d")]
        #[allow(non_camel_case_types)]
        Threed,
        #[serde(rename = "1w")]
        #[allow(non_camel_case_types)]
        Onew,
        #[serde(rename = "1M")]
        #[allow(non_camel_case_types)]
        OneM,
    }

    impl std::fmt::Display for Interval {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Interval::Onem => "1m",
                    Interval::Threem => "3m",
                    Interval::Fivem => "5m",
                    Interval::Fifteenm => "15m",
                    Interval::Thirtym => "30m",
                    Interval::Oneh => "1h",
                    Interval::Twoh => "2h",
                    Interval::Fourh => "4h",
                    Interval::Sixh => "6h",
                    Interval::Eighth => "8h",
                    Interval::Twelveh => "12h",
                    Interval::Oned => "1d",
                    Interval::Threed => "3d",
                    Interval::Onew => "1w",
                    Interval::OneM => "1M",
                }
            )
        }
    }

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    /// Query parameters for get_api_v3_klines operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  kline intervals
        #[serde(rename = "interval")]
        pub interval: Interval,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  kline data
    pub type Response200 = Vec<f64>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  kline data
    pub type Status200 = Vec<f64>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_api_v3_my_trades {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_my_trades operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                start_time: query.start_time,
                end_time: query.end_time,
                from_id: query.from_id,
                limit: query.limit,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                from_id: self.from_id.clone(),
                limit: self.limit.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Trade id to fetch from. Default gets most recent trades.
    pub type FromId = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_my_trades operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(rename = "from_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  order details
    pub type Response200 = components::schemas::MyTrade;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  order details
    pub type Status200 = components::schemas::MyTrade;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_open_order_list {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_open_order_list operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_open_order_list operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  oco order list
    pub type Response200 = Vec<components::schemas::OcoOrder>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  oco order list
    pub type Status200 = Vec<components::schemas::OcoOrder>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_open_orders {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_open_orders operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_open_orders operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  current open orders
    pub type Response200 = Vec<components::schemas::OrderDetails>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  current open orders
    pub type Status200 = Vec<components::schemas::OrderDetails>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod delete_api_v3_open_orders {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for delete_api_v3_open_orders operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for delete_api_v3_open_orders operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  cancelled orders
    pub type Response200 = Vec<components::schemas::Order>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  cancelled orders
    pub type Status200 = Vec<components::schemas::Order>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_order {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_order operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  order id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                order_id: query.order_id,
                orig_client_order_id: query.orig_client_order_id,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                order_id: self.order_id.clone(),
                orig_client_order_id: self.orig_client_order_id.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  order id
    pub type OrderId = i64;

    ///  order id from client
    pub type OrigClientOrderId = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_order operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  order id
        #[serde(rename = "order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(rename = "orig_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  order details
    pub type Response200 = components::schemas::OrderDetails;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  order details
    pub type Status200 = components::schemas::OrderDetails;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_api_v3_order {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_api_v3_order operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  SELL or BUY
        pub side: Side,
        ///  the order type
        #[serde(rename = "type")]
        pub type_: Type,
        ///  order time in force
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<TimeInForce>,
        ///  order quantity
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quantity: Option<f32>,
        ///  quote quantity
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_order_qty: Option<f32>,
        ///  order price
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f32>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<f32>,
        ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_qty: Option<f32>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                side: query.side,
                type_: query.type_,
                time_in_force: query.time_in_force,
                quantity: query.quantity,
                quote_order_qty: query.quote_order_qty,
                price: query.price,
                new_client_order_id: query.new_client_order_id,
                stop_price: query.stop_price,
                iceberg_qty: query.iceberg_qty,
                new_order_resp_type: query.new_order_resp_type,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                side: self.side.clone(),
                type_: self.type_.clone(),
                time_in_force: self.time_in_force.clone(),
                quantity: self.quantity.clone(),
                quote_order_qty: self.quote_order_qty.clone(),
                price: self.price.clone(),
                new_client_order_id: self.new_client_order_id.clone(),
                stop_price: self.stop_price.clone(),
                iceberg_qty: self.iceberg_qty.clone(),
                new_order_resp_type: self.new_order_resp_type.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  SELL or BUY

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Side {
        #[serde(rename = "SELL")]
        #[allow(non_camel_case_types)]
        Sell,
        #[serde(rename = "BUY")]
        #[allow(non_camel_case_types)]
        Buy,
    }

    impl std::fmt::Display for Side {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Side::Sell => "SELL",
                    Side::Buy => "BUY",
                }
            )
        }
    }

    ///  the order type

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Type {
        #[serde(rename = "LIMIT")]
        #[allow(non_camel_case_types)]
        Limit,
        #[serde(rename = "MARKET")]
        #[allow(non_camel_case_types)]
        Market,
        #[serde(rename = "STOP_LOSS")]
        #[allow(non_camel_case_types)]
        StopLoss,
        #[serde(rename = "STOP_LOSS_LIMIT")]
        #[allow(non_camel_case_types)]
        StopLossLimit,
        #[serde(rename = "TAKE_PROFIT")]
        #[allow(non_camel_case_types)]
        TakeProfit,
        #[serde(rename = "TAKE_PROFIT_LIMIT")]
        #[allow(non_camel_case_types)]
        TakeProfitLimit,
        #[serde(rename = "LIMIT_MAKER")]
        #[allow(non_camel_case_types)]
        LimitMaker,
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Type::Limit => "LIMIT",
                    Type::Market => "MARKET",
                    Type::StopLoss => "STOP_LOSS",
                    Type::StopLossLimit => "STOP_LOSS_LIMIT",
                    Type::TakeProfit => "TAKE_PROFIT",
                    Type::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
                    Type::LimitMaker => "LIMIT_MAKER",
                }
            )
        }
    }

    ///  order time in force

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum TimeInForce {
        #[serde(rename = "GTC")]
        #[allow(non_camel_case_types)]
        Gtc,
        #[serde(rename = "IOC")]
        #[allow(non_camel_case_types)]
        Ioc,
        #[serde(rename = "FOK")]
        #[allow(non_camel_case_types)]
        Fok,
    }

    impl std::fmt::Display for TimeInForce {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    TimeInForce::Gtc => "GTC",
                    TimeInForce::Ioc => "IOC",
                    TimeInForce::Fok => "FOK",
                }
            )
        }
    }

    ///  order quantity
    pub type Quantity = f32;

    ///  quote quantity
    pub type QuoteOrderQty = f32;

    ///  order price
    pub type Price = f32;

    ///  Used to uniquely identify this cancel. Automatically generated by default
    pub type NewClientOrderId = String;

    ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
    pub type StopPrice = f32;

    ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
    pub type IcebergQty = f32;

    ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum NewOrderRespType {
        #[serde(rename = "ACK")]
        #[allow(non_camel_case_types)]
        Ack,
        #[serde(rename = "RESULT")]
        #[allow(non_camel_case_types)]
        Result,
        #[serde(rename = "FULL")]
        #[allow(non_camel_case_types)]
        Full,
    }

    impl std::fmt::Display for NewOrderRespType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    NewOrderRespType::Ack => "ACK",
                    NewOrderRespType::Result => "RESULT",
                    NewOrderRespType::Full => "FULL",
                }
            )
        }
    }

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_api_v3_order operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  SELL or BUY
        #[serde(rename = "side")]
        pub side: Side,
        ///  the order type
        #[serde(rename = "type")]
        pub type_: Type,
        ///  order time in force
        #[serde(rename = "time_in_force")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<TimeInForce>,
        ///  order quantity
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quantity: Option<f32>,
        ///  quote quantity
        #[serde(rename = "quote_order_qty")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_order_qty: Option<f32>,
        ///  order price
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f32>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(rename = "new_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
        #[serde(rename = "stop_price")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<f32>,
        ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
        #[serde(rename = "iceberg_qty")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_qty: Option<f32>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(rename = "new_order_resp_type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  order result
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  order result
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod delete_api_v3_order {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for delete_api_v3_order operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  order id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                order_id: query.order_id,
                orig_client_order_id: query.orig_client_order_id,
                new_client_order_id: query.new_client_order_id,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                order_id: self.order_id.clone(),
                orig_client_order_id: self.orig_client_order_id.clone(),
                new_client_order_id: self.new_client_order_id.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  order id
    pub type OrderId = i64;

    ///  order id from client
    pub type OrigClientOrderId = String;

    ///  Used to uniquely identify this cancel. Automatically generated by default
    pub type NewClientOrderId = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for delete_api_v3_order operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  order id
        #[serde(rename = "order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(rename = "orig_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(rename = "new_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  cancelled order
    pub type Response200 = components::schemas::Order;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  cancelled order
    pub type Status200 = components::schemas::Order;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_api_v3_order_oco {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_api_v3_order_oco operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  A unique Id for the entire orderList
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_client_order_id: Option<String>,
        ///  SELL or BUY
        pub side: Side,
        ///  order quantity
        pub quantity: f32,
        ///  A unique Id for the limit order
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit_client_order_id: Option<f32>,
        pub price: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit_iceberg_qty: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_client_order_id: Option<String>,
        pub stop_price: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_limit_price: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_iceberg_qty: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_limit_time_in_force: Option<StopLimitTimeInForce>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                list_client_order_id: query.list_client_order_id,
                side: query.side,
                quantity: query.quantity,
                limit_client_order_id: query.limit_client_order_id,
                price: query.price,
                limit_iceberg_qty: query.limit_iceberg_qty,
                stop_client_order_id: query.stop_client_order_id,
                stop_price: query.stop_price,
                stop_limit_price: query.stop_limit_price,
                stop_iceberg_qty: query.stop_iceberg_qty,
                stop_limit_time_in_force: query.stop_limit_time_in_force,
                new_order_resp_type: query.new_order_resp_type,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                list_client_order_id: self.list_client_order_id.clone(),
                side: self.side.clone(),
                quantity: self.quantity.clone(),
                limit_client_order_id: self.limit_client_order_id.clone(),
                price: self.price.clone(),
                limit_iceberg_qty: self.limit_iceberg_qty.clone(),
                stop_client_order_id: self.stop_client_order_id.clone(),
                stop_price: self.stop_price.clone(),
                stop_limit_price: self.stop_limit_price.clone(),
                stop_iceberg_qty: self.stop_iceberg_qty.clone(),
                stop_limit_time_in_force: self.stop_limit_time_in_force.clone(),
                new_order_resp_type: self.new_order_resp_type.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  A unique Id for the entire orderList
    pub type ListClientOrderId = String;

    ///  SELL or BUY

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Side {
        #[serde(rename = "SELL")]
        #[allow(non_camel_case_types)]
        Sell,
        #[serde(rename = "BUY")]
        #[allow(non_camel_case_types)]
        Buy,
    }

    impl std::fmt::Display for Side {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Side::Sell => "SELL",
                    Side::Buy => "BUY",
                }
            )
        }
    }

    ///  order quantity
    pub type Quantity = f32;

    ///  A unique Id for the limit order
    pub type LimitClientOrderId = f32;

    pub type Price = f32;

    pub type LimitIcebergQty = f32;

    pub type StopClientOrderId = String;

    pub type StopPrice = f32;

    pub type StopLimitPrice = f32;

    pub type StopIcebergQty = f32;

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum StopLimitTimeInForce {
        #[serde(rename = "GTC")]
        #[allow(non_camel_case_types)]
        Gtc,
        #[serde(rename = "FOK")]
        #[allow(non_camel_case_types)]
        Fok,
        #[serde(rename = "IOC")]
        #[allow(non_camel_case_types)]
        Ioc,
    }

    impl std::fmt::Display for StopLimitTimeInForce {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    StopLimitTimeInForce::Gtc => "GTC",
                    StopLimitTimeInForce::Fok => "FOK",
                    StopLimitTimeInForce::Ioc => "IOC",
                }
            )
        }
    }

    ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum NewOrderRespType {
        #[serde(rename = "ACK")]
        #[allow(non_camel_case_types)]
        Ack,
        #[serde(rename = "RESULT")]
        #[allow(non_camel_case_types)]
        Result,
        #[serde(rename = "FULL")]
        #[allow(non_camel_case_types)]
        Full,
    }

    impl std::fmt::Display for NewOrderRespType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    NewOrderRespType::Ack => "ACK",
                    NewOrderRespType::Result => "RESULT",
                    NewOrderRespType::Full => "FULL",
                }
            )
        }
    }

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_api_v3_order_oco operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  A unique Id for the entire orderList
        #[serde(rename = "list_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_client_order_id: Option<String>,
        ///  SELL or BUY
        #[serde(rename = "side")]
        pub side: Side,
        ///  order quantity
        #[serde(rename = "quantity")]
        pub quantity: f32,
        ///  A unique Id for the limit order
        #[serde(rename = "limit_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit_client_order_id: Option<f32>,

        #[serde(rename = "price")]
        pub price: f32,

        #[serde(rename = "limit_iceberg_qty")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit_iceberg_qty: Option<f32>,

        #[serde(rename = "stop_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_client_order_id: Option<String>,

        #[serde(rename = "stop_price")]
        pub stop_price: f32,

        #[serde(rename = "stop_limit_price")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_limit_price: Option<f32>,

        #[serde(rename = "stop_iceberg_qty")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_iceberg_qty: Option<f32>,

        #[serde(rename = "stop_limit_time_in_force")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_limit_time_in_force: Option<StopLimitTimeInForce>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(rename = "new_order_resp_type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200OrderReportsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub client_order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cummulative_quote_qty: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub executed_qty: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_qty: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub side: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transact_time: Option<i64>,
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200OrdersItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub client_order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    ///  new oco details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contingency_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_client_order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_order_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_status_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_reports: Option<Vec<Response200OrderReportsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orders: Option<Vec<Response200OrdersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_time: Option<i64>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200OrderReportsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub client_order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cummulative_quote_qty: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub executed_qty: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_qty: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub side: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transact_time: Option<i64>,
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200OrdersItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub client_order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    ///  new oco details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contingency_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_client_order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_order_status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_status_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_reports: Option<Vec<Status200OrderReportsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orders: Option<Vec<Status200OrdersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_time: Option<i64>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_api_v3_order_test {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_api_v3_order_test operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  SELL or BUY
        pub side: Side,
        ///  the order type
        #[serde(rename = "type")]
        pub type_: Type,
        ///  order time in force
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<TimeInForce>,
        ///  order quantity
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quantity: Option<f32>,
        ///  quote quantity
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_order_qty: Option<f32>,
        ///  order price
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f32>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<f32>,
        ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_qty: Option<f32>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                side: query.side,
                type_: query.type_,
                time_in_force: query.time_in_force,
                quantity: query.quantity,
                quote_order_qty: query.quote_order_qty,
                price: query.price,
                new_client_order_id: query.new_client_order_id,
                stop_price: query.stop_price,
                iceberg_qty: query.iceberg_qty,
                new_order_resp_type: query.new_order_resp_type,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                side: self.side.clone(),
                type_: self.type_.clone(),
                time_in_force: self.time_in_force.clone(),
                quantity: self.quantity.clone(),
                quote_order_qty: self.quote_order_qty.clone(),
                price: self.price.clone(),
                new_client_order_id: self.new_client_order_id.clone(),
                stop_price: self.stop_price.clone(),
                iceberg_qty: self.iceberg_qty.clone(),
                new_order_resp_type: self.new_order_resp_type.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  SELL or BUY

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Side {
        #[serde(rename = "SELL")]
        #[allow(non_camel_case_types)]
        Sell,
        #[serde(rename = "BUY")]
        #[allow(non_camel_case_types)]
        Buy,
    }

    impl std::fmt::Display for Side {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Side::Sell => "SELL",
                    Side::Buy => "BUY",
                }
            )
        }
    }

    ///  the order type

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Type {
        #[serde(rename = "LIMIT")]
        #[allow(non_camel_case_types)]
        Limit,
        #[serde(rename = "MARKET")]
        #[allow(non_camel_case_types)]
        Market,
        #[serde(rename = "STOP_LOSS")]
        #[allow(non_camel_case_types)]
        StopLoss,
        #[serde(rename = "STOP_LOSS_LIMIT")]
        #[allow(non_camel_case_types)]
        StopLossLimit,
        #[serde(rename = "TAKE_PROFIT")]
        #[allow(non_camel_case_types)]
        TakeProfit,
        #[serde(rename = "TAKE_PROFIT_LIMIT")]
        #[allow(non_camel_case_types)]
        TakeProfitLimit,
        #[serde(rename = "LIMIT_MAKER")]
        #[allow(non_camel_case_types)]
        LimitMaker,
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Type::Limit => "LIMIT",
                    Type::Market => "MARKET",
                    Type::StopLoss => "STOP_LOSS",
                    Type::StopLossLimit => "STOP_LOSS_LIMIT",
                    Type::TakeProfit => "TAKE_PROFIT",
                    Type::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
                    Type::LimitMaker => "LIMIT_MAKER",
                }
            )
        }
    }

    ///  order time in force

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum TimeInForce {
        #[serde(rename = "GTC")]
        #[allow(non_camel_case_types)]
        Gtc,
        #[serde(rename = "IOC")]
        #[allow(non_camel_case_types)]
        Ioc,
        #[serde(rename = "FOK")]
        #[allow(non_camel_case_types)]
        Fok,
    }

    impl std::fmt::Display for TimeInForce {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    TimeInForce::Gtc => "GTC",
                    TimeInForce::Ioc => "IOC",
                    TimeInForce::Fok => "FOK",
                }
            )
        }
    }

    ///  order quantity
    pub type Quantity = f32;

    ///  quote quantity
    pub type QuoteOrderQty = f32;

    ///  order price
    pub type Price = f32;

    ///  Used to uniquely identify this cancel. Automatically generated by default
    pub type NewClientOrderId = String;

    ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
    pub type StopPrice = f32;

    ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
    pub type IcebergQty = f32;

    ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum NewOrderRespType {
        #[serde(rename = "ACK")]
        #[allow(non_camel_case_types)]
        Ack,
        #[serde(rename = "RESULT")]
        #[allow(non_camel_case_types)]
        Result,
        #[serde(rename = "FULL")]
        #[allow(non_camel_case_types)]
        Full,
    }

    impl std::fmt::Display for NewOrderRespType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    NewOrderRespType::Ack => "ACK",
                    NewOrderRespType::Result => "RESULT",
                    NewOrderRespType::Full => "FULL",
                }
            )
        }
    }

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_api_v3_order_test operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  SELL or BUY
        #[serde(rename = "side")]
        pub side: Side,
        ///  the order type
        #[serde(rename = "type")]
        pub type_: Type,
        ///  order time in force
        #[serde(rename = "time_in_force")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<TimeInForce>,
        ///  order quantity
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quantity: Option<f32>,
        ///  quote quantity
        #[serde(rename = "quote_order_qty")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote_order_qty: Option<f32>,
        ///  order price
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f32>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(rename = "new_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
        #[serde(rename = "stop_price")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<f32>,
        ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
        #[serde(rename = "iceberg_qty")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_qty: Option<f32>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(rename = "new_order_resp_type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  test order result
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  test order result
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_order_list {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_order_list operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  order list id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        ///  order id from client
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                order_list_id: query.order_list_id,
                orig_client_order_id: query.orig_client_order_id,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                order_list_id: self.order_list_id.clone(),
                orig_client_order_id: self.orig_client_order_id.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  order list id
    pub type OrderListId = i64;

    ///  order id from client
    pub type OrigClientOrderId = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_api_v3_order_list operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  order list id
        #[serde(rename = "order_list_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        ///  order id from client
        #[serde(rename = "orig_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  oco details
    pub type Response200 = components::schemas::OcoOrder;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  oco details
    pub type Status200 = components::schemas::OcoOrder;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod delete_api_v3_order_list {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for delete_api_v3_order_list operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  order list id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        ///  A unique Id for the entire orderList
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_client_order_id: Option<String>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                order_list_id: query.order_list_id,
                list_client_order_id: query.list_client_order_id,
                new_client_order_id: query.new_client_order_id,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                order_list_id: self.order_list_id.clone(),
                list_client_order_id: self.list_client_order_id.clone(),
                new_client_order_id: self.new_client_order_id.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  order list id
    pub type OrderListId = i64;

    ///  A unique Id for the entire orderList
    pub type ListClientOrderId = String;

    ///  Used to uniquely identify this cancel. Automatically generated by default
    pub type NewClientOrderId = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for delete_api_v3_order_list operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  order list id
        #[serde(rename = "order_list_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_list_id: Option<i64>,
        ///  A unique Id for the entire orderList
        #[serde(rename = "list_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub list_client_order_id: Option<String>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(rename = "new_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  oco order delete report
    pub type Response200 = components::schemas::OcoOrderReport;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  oco order delete report
    pub type Status200 = components::schemas::OcoOrderReport;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_api_v3_ping {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_ping operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    ///  ping result
    pub type Response200 = ();

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  ping result
    pub type Status200 = ();
}

pub mod get_api_v3_ticker_24hr {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_ticker_24hr operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    /// Query parameters for get_api_v3_ticker_24hr operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  24hr ticker
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  24hr ticker
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_api_v3_ticker_book_ticker {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_ticker_book_ticker operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    /// Query parameters for get_api_v3_ticker_book_ticker operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  orderbook ticker
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  orderbook ticker
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_api_v3_ticker_price {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_ticker_price operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    /// Query parameters for get_api_v3_ticker_price operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  Price ticker
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  Price ticker
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_api_v3_time {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_time operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    ///  Binance server UTC timestamp
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub server_time: Option<i64>,
    }

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  Binance server UTC timestamp
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub server_time: Option<i64>,
    }
}

pub mod get_api_v3_trades {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_api_v3_trades operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                limit: query.limit,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                limit: self.limit.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    pub type Limit = i64;

    /// Query parameters for get_api_v3_trades operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,

        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  trade list
    pub type Response200 = Vec<components::schemas::Trade>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  trade list
    pub type Status200 = Vec<components::schemas::Trade>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod post_api_v3_user_data_stream {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_api_v3_user_data_stream operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response401(Response401),
        Unspecified(T),
    }

    ///  listen key
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  listen key
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod put_api_v3_user_data_stream {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for put_api_v3_user_data_stream operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  User websocket listen key
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                listen_key: query.listen_key,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                listen_key: self.listen_key.clone(),
            }
        }
    }
    ///  User websocket listen key
    pub type ListenKey = String;

    /// Query parameters for put_api_v3_user_data_stream operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  User websocket listen key
        #[serde(rename = "listen_key")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  empty object

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  empty object

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod delete_api_v3_user_data_stream {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for delete_api_v3_user_data_stream operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  User websocket listen key
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                listen_key: query.listen_key,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                listen_key: self.listen_key.clone(),
            }
        }
    }
    ///  User websocket listen key
    pub type ListenKey = String;

    /// Query parameters for delete_api_v3_user_data_stream operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  User websocket listen key
        #[serde(rename = "listen_key")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  empty object

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  empty object

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_sapi_v1_account_disable_fast_withdraw_switch {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_account_disable_fast_withdraw_switch operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_account_disable_fast_withdraw_switch operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  return empty
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  return empty
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_sapi_v1_account_enable_fast_withdraw_switch {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_account_enable_fast_withdraw_switch operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_account_enable_fast_withdraw_switch operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  return empty
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  return empty
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_account_snapshot {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_account_snapshot operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        #[serde(rename = "type")]
        pub type_: Type,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        pub limit: i64,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                type_: query.type_,
                start_time: query.start_time,
                end_time: query.end_time,
                limit: query.limit,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                type_: self.type_.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                limit: self.limit.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Type {
        #[serde(rename = "SPOT")]
        #[allow(non_camel_case_types)]
        Spot,
        #[serde(rename = "MARGIN")]
        #[allow(non_camel_case_types)]
        Margin,
        #[serde(rename = "FUTURES")]
        #[allow(non_camel_case_types)]
        Futures,
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Type::Spot => "SPOT",
                    Type::Margin => "MARGIN",
                    Type::Futures => "FUTURES",
                }
            )
        }
    }

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    pub type Limit = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_account_snapshot operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        #[serde(rename = "type")]
        pub type_: Type,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,

        #[serde(rename = "limit")]
        pub limit: i64,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200SnapshotVosItemDataBalancesItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub free: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locked: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200SnapshotVosItemData {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub balances: Option<Vec<Response200SnapshotVosItemDataBalancesItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_asset_of_btc: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200SnapshotVosItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Response200SnapshotVosItemData>,
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub update_time: Option<i64>,
    }

    ///  account Snapshot
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub msg: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub snapshot_vos: Option<Vec<Response200SnapshotVosItem>>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200SnapshotVosItemDataBalancesItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub free: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locked: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200SnapshotVosItemData {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub balances: Option<Vec<Status200SnapshotVosItemDataBalancesItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_asset_of_btc: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200SnapshotVosItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Status200SnapshotVosItemData>,
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub update_time: Option<i64>,
    }

    ///  account Snapshot
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub msg: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub snapshot_vos: Option<Vec<Status200SnapshotVosItem>>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_asset_asset_dividend {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_asset_asset_dividend operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        pub limit: String,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                start_time: query.start_time,
                end_time: query.end_time,
                limit: query.limit,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                limit: self.limit.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    pub type Limit = String;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_asset_asset_dividend operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,

        #[serde(rename = "limit")]
        pub limit: String,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200RowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub div_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub en_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
    }

    ///  records of asset devidend
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Response200RowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200RowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub div_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub en_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
    }

    ///  records of asset devidend
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Status200RowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_sapi_v1_asset_dust {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_asset_dust operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being converted. For example, asset=BTC&asset=USDT
        pub asset: String,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being converted. For example, asset=BTC&asset=USDT
    pub type Asset = String;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_asset_dust operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being converted. For example, asset=BTC&asset=USDT
        #[serde(rename = "asset")]
        pub asset: String,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200TransferResultItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operate_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_charge_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfered_amount: Option<String>,
    }

    ///  Dust log records
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_service_charge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_transfered: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_result: Option<Vec<Response200TransferResultItem>>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200TransferResultItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operate_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_charge_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfered_amount: Option<String>,
    }

    ///  Dust log records
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_service_charge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_transfered: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_result: Option<Vec<Status200TransferResultItem>>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_capital_config_getall {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_capital_config_getall operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_capital_config_getall operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200NetworkListItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address_regex: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_desc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_default: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memo_regex: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_confirm: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reset_address_status: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub special_tips: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub un_lock_confirm: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_desc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_fee: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_min: Option<String>,
    }

    ///  All coins details information
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_all_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub free: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub freeze: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ipoable: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ipoing: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_legal_money: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locked: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network_list: Option<Vec<Response200NetworkListItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub storage: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trading: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_all_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdrawing: Option<String>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200NetworkListItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address_regex: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_desc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_default: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memo_regex: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_confirm: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reset_address_status: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub special_tips: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub un_lock_confirm: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_desc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_fee: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_min: Option<String>,
    }

    ///  All coins details information
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_all_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub free: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub freeze: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ipoable: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ipoing: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_legal_money: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locked: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network_list: Option<Vec<Status200NetworkListItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub storage: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trading: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_all_enable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdrawing: Option<String>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_capital_deposit_address {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_capital_deposit_address operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  coin name
        pub coin: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                coin: query.coin,
                network: query.network,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                coin: self.coin.clone(),
                network: self.network.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  coin name
    pub type Coin = String;

    pub type Network = String;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_capital_deposit_address operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  coin name
        #[serde(rename = "coin")]
        pub coin: String,

        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  deposit address
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  deposit address
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_capital_deposit_hisrec {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_capital_deposit_hisrec operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        ///  0 -> pending
        ///  6 -> credited but cannot withdraw
        ///  1 -> success
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                coin: query.coin,
                status: query.status,
                start_time: query.start_time,
                end_time: query.end_time,
                offset: query.offset,
                limit: query.limit,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                coin: self.coin.clone(),
                status: self.status.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                offset: self.offset.clone(),
                limit: self.limit.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    pub type Coin = String;

    ///  0 -> pending
    ///  6 -> credited but cannot withdraw
    ///  1 -> success
    pub type Status = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    pub type Offset = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_capital_deposit_hisrec operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        #[serde(rename = "coin")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        ///  0 -> pending
        ///  6 -> credited but cannot withdraw
        ///  1 -> success
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,

        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub insert_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<String>,
    }

    pub type Response200 = Vec<Response200Item>;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address_tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub insert_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<String>,
    }

    pub type Status200 = Vec<Status200Item>;
}

pub mod post_sapi_v1_capital_withdraw_apply {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_capital_withdraw_apply operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  coin name
        pub coin: String,
        ///  client id for withdraw
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_order_id: Option<String>,
        ///  getting value from `GET /sapi/v1/capital/config/getall`
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        pub address: String,
        ///  Secondary address identifier for coins like XRP,XMR etc.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address_tag: Option<String>,
        pub amount: f32,
        ///  When making internal transfer
        ///  - `true` ->  returning the fee to the destination account;
        ///  - `false` -> returning the fee back to the departure account.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_fee_flag: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                coin: query.coin,
                withdraw_order_id: query.withdraw_order_id,
                network: query.network,
                address: query.address,
                address_tag: query.address_tag,
                amount: query.amount,
                transaction_fee_flag: query.transaction_fee_flag,
                name: query.name,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                coin: self.coin.clone(),
                withdraw_order_id: self.withdraw_order_id.clone(),
                network: self.network.clone(),
                address: self.address.clone(),
                address_tag: self.address_tag.clone(),
                amount: self.amount.clone(),
                transaction_fee_flag: self.transaction_fee_flag.clone(),
                name: self.name.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  coin name
    pub type Coin = String;

    ///  client id for withdraw
    pub type WithdrawOrderId = String;

    ///  getting value from `GET /sapi/v1/capital/config/getall`
    pub type Network = String;

    pub type Address = String;

    ///  Secondary address identifier for coins like XRP,XMR etc.
    pub type AddressTag = String;

    pub type Amount = f32;

    ///  When making internal transfer
    ///  - `true` ->  returning the fee to the destination account;
    ///  - `false` -> returning the fee back to the departure account.
    pub type TransactionFeeFlag = bool;

    pub type Name = String;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_capital_withdraw_apply operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  coin name
        #[serde(rename = "coin")]
        pub coin: String,
        ///  client id for withdraw
        #[serde(rename = "withdraw_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_order_id: Option<String>,
        ///  getting value from `GET /sapi/v1/capital/config/getall`
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,

        #[serde(rename = "address")]
        pub address: String,
        ///  Secondary address identifier for coins like XRP,XMR etc.
        #[serde(rename = "address_tag")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address_tag: Option<String>,

        #[serde(rename = "amount")]
        pub amount: f32,
        ///  When making internal transfer
        ///  - `true` ->  returning the fee to the destination account;
        ///  - `false` -> returning the fee back to the departure account.
        #[serde(rename = "transaction_fee_flag")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_fee_flag: Option<bool>,

        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_capital_withdraw_history {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_capital_withdraw_history operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        ///  0:Email Sent
        ///  1:Cancelled
        ///  2:Awaiting Approval
        ///  3:Rejected
        ///  4:Processing
        ///  5:Failure
        ///  6:Completed
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                coin: query.coin,
                status: query.status,
                start_time: query.start_time,
                end_time: query.end_time,
                offset: query.offset,
                limit: query.limit,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                coin: self.coin.clone(),
                status: self.status.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                offset: self.offset.clone(),
                limit: self.limit.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    pub type Coin = String;

    ///  0:Email Sent
    ///  1:Cancelled
    ///  2:Awaiting Approval
    ///  3:Rejected
    ///  4:Processing
    ///  5:Failure
    ///  6:Completed
    pub type Status = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    pub type Offset = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_capital_withdraw_history operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        #[serde(rename = "coin")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        ///  0:Email Sent
        ///  1:Cancelled
        ///  2:Awaiting Approval
        ///  3:Rejected
        ///  4:Processing
        ///  5:Failure
        ///  6:Completed
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,

        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub apply_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<String>,
    }

    pub type Response200 = Vec<Response200Item>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub apply_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub network: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<String>,
    }

    pub type Status200 = Vec<Status200Item>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_account {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_account operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_account operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200UserAssetsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub borrowed: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub free: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locked: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub net_asset: Option<String>,
    }

    ///  Margin account details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub borrow_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub margin_level: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_asset_of_btc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_liability_of_btc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_net_asset_of_btc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trade_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_assets: Option<Vec<Response200UserAssetsItem>>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200UserAssetsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub borrowed: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub free: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locked: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub net_asset: Option<String>,
    }

    ///  Margin account details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub borrow_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub margin_level: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_asset_of_btc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_liability_of_btc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_net_asset_of_btc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trade_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfer_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_assets: Option<Vec<Status200UserAssetsItem>>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_all_assets {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_all_assets operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_full_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_borrowable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_mortgageable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_borrow: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_repay: Option<String>,
    }

    ///  assets details
    pub type Response200 = Vec<Response200Item>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_full_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_borrowable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_mortgageable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_borrow: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_repay: Option<String>,
    }

    ///  assets details
    pub type Status200 = Vec<Status200Item>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_all_orders {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_all_orders operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  order id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                order_id: query.order_id,
                start_time: query.start_time,
                end_time: query.end_time,
                limit: query.limit,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                order_id: self.order_id.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                limit: self.limit.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  order id
    pub type OrderId = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_all_orders operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  order id
        #[serde(rename = "order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  margin order list
    pub type Response200 = Vec<components::schemas::MarginOrderDetail>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  margin order list
    pub type Status200 = Vec<components::schemas::MarginOrderDetail>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_all_pairs {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_all_pairs operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_buy_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_margin_trade: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_sell_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    ///  margin pairs
    pub type Response200 = Vec<Response200Item>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_buy_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_margin_trade: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_sell_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }

    ///  margin pairs
    pub type Status200 = Vec<Status200Item>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_asset {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_asset operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        pub asset: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self { asset: query.asset }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    /// Query parameters for get_sapi_v1_margin_asset operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        pub asset: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  asset details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_full_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_borrowable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_mortgageable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_borrow: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_repay: Option<String>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  asset details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_full_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_borrowable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_mortgageable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_borrow: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_min_repay: Option<String>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_force_liquidation_rec {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_force_liquidation_rec operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                start_time: query.start_time,
                end_time: query.end_time,
                current: query.current,
                size: query.size,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                current: self.current.clone(),
                size: self.size.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Currently querying page. Start from 1. Default:1
    pub type Current = f64;

    ///  Default:10 Max:100
    pub type Size = f64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_force_liquidation_rec operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(rename = "current")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    ///  Force Liquidation History, response in descending order

    pub type Response200 = ();

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  Force Liquidation History, response in descending order

    pub type Status200 = ();
}

pub mod get_sapi_v1_margin_interest_history {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_interest_history operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                start_time: query.start_time,
                end_time: query.end_time,
                current: query.current,
                size: query.size,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                current: self.current.clone(),
                size: self.size.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Currently querying page. Start from 1. Default:1
    pub type Current = f64;

    ///  Default:10 Max:100
    pub type Size = f64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_interest_history operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(rename = "current")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200RowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest_accured_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest_rate: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub principal: Option<String>,
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    ///  Interest History, response in descending order
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Response200RowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200RowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest_accured_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest_rate: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub principal: Option<String>,
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    ///  Interest History, response in descending order
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Status200RowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_loan {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_loan operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        pub asset: String,
        ///  the tranId in  `POST /sapi/v1/margin/loan`
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                tx_id: query.tx_id,
                start_time: query.start_time,
                end_time: query.end_time,
                current: query.current,
                size: query.size,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                tx_id: self.tx_id.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                current: self.current.clone(),
                size: self.size.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    ///  the tranId in  `POST /sapi/v1/margin/loan`
    pub type TxId = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Currently querying page. Start from 1. Default:1
    pub type Current = f64;

    ///  Default:10 Max:100
    pub type Size = f64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_loan operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        pub asset: String,
        ///  the tranId in  `POST /sapi/v1/margin/loan`
        #[serde(rename = "tx_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(rename = "current")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  Load records

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  Load records

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_sapi_v1_margin_loan {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_margin_loan operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        pub asset: String,
        pub amount: f32,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                amount: query.amount,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                amount: self.amount.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    pub type Amount = f32;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_margin_loan operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        pub asset: String,

        #[serde(rename = "amount")]
        pub amount: f32,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  order details
    pub type Response200 = components::schemas::Transaction;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  order details
    pub type Status200 = components::schemas::Transaction;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_max_borrowable {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_max_borrowable operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_max_borrowable operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  max borrow amount

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  max borrow amount

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_max_transferable {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_max_transferable operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_max_transferable operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  max transferable amount

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  max transferable amount

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_my_trades {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_my_trades operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                start_time: query.start_time,
                end_time: query.end_time,
                from_id: query.from_id,
                limit: query.limit,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                from_id: self.from_id.clone(),
                limit: self.limit.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Trade id to fetch from. Default gets most recent trades.
    pub type FromId = i64;

    ///  Default 500; max 1000.
    pub type Limit = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_my_trades operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Trade id to fetch from. Default gets most recent trades.
        #[serde(rename = "from_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<i64>,
        ///  Default 500; max 1000.
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  margin order list
    pub type Response200 = Vec<components::schemas::MarginTrade>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  margin order list
    pub type Status200 = Vec<components::schemas::MarginTrade>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_open_orders {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_open_orders operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_open_orders operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  margin open orders list
    pub type Response200 = Vec<components::schemas::MarginOrderDetail>;

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  margin open orders list
    pub type Status200 = Vec<components::schemas::MarginOrderDetail>;

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_order {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_order operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  order id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                order_id: query.order_id,
                orig_client_order_id: query.orig_client_order_id,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                order_id: self.order_id.clone(),
                orig_client_order_id: self.orig_client_order_id.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  order id
    pub type OrderId = i64;

    ///  order id from client
    pub type OrigClientOrderId = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_order operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  order id
        #[serde(rename = "order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(rename = "orig_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  Interest History, response in descending order
    pub type Response200 = components::schemas::MarginOrderDetail;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  Interest History, response in descending order
    pub type Status200 = components::schemas::MarginOrderDetail;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_sapi_v1_margin_order {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_margin_order operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  SELL or BUY
        pub side: Side,
        ///  the order type
        #[serde(rename = "type")]
        pub type_: Type,
        ///  order quantity
        pub quantity: f32,
        ///  order price
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f32>,
        ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<f32>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_qty: Option<f32>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  default NO_SIDE_EFFECT
        #[serde(skip_serializing_if = "Option::is_none")]
        pub side_effect_type: Option<SideEffectType>,
        ///  order time in force
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<TimeInForce>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                side: query.side,
                type_: query.type_,
                quantity: query.quantity,
                price: query.price,
                stop_price: query.stop_price,
                new_client_order_id: query.new_client_order_id,
                iceberg_qty: query.iceberg_qty,
                new_order_resp_type: query.new_order_resp_type,
                side_effect_type: query.side_effect_type,
                time_in_force: query.time_in_force,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                side: self.side.clone(),
                type_: self.type_.clone(),
                quantity: self.quantity.clone(),
                price: self.price.clone(),
                stop_price: self.stop_price.clone(),
                new_client_order_id: self.new_client_order_id.clone(),
                iceberg_qty: self.iceberg_qty.clone(),
                new_order_resp_type: self.new_order_resp_type.clone(),
                side_effect_type: self.side_effect_type.clone(),
                time_in_force: self.time_in_force.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  SELL or BUY

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Side {
        #[serde(rename = "SELL")]
        #[allow(non_camel_case_types)]
        Sell,
        #[serde(rename = "BUY")]
        #[allow(non_camel_case_types)]
        Buy,
    }

    impl std::fmt::Display for Side {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Side::Sell => "SELL",
                    Side::Buy => "BUY",
                }
            )
        }
    }

    ///  the order type

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Type {
        #[serde(rename = "LIMIT")]
        #[allow(non_camel_case_types)]
        Limit,
        #[serde(rename = "MARKET")]
        #[allow(non_camel_case_types)]
        Market,
        #[serde(rename = "STOP_LOSS")]
        #[allow(non_camel_case_types)]
        StopLoss,
        #[serde(rename = "STOP_LOSS_LIMIT")]
        #[allow(non_camel_case_types)]
        StopLossLimit,
        #[serde(rename = "TAKE_PROFIT")]
        #[allow(non_camel_case_types)]
        TakeProfit,
        #[serde(rename = "TAKE_PROFIT_LIMIT")]
        #[allow(non_camel_case_types)]
        TakeProfitLimit,
        #[serde(rename = "LIMIT_MAKER")]
        #[allow(non_camel_case_types)]
        LimitMaker,
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Type::Limit => "LIMIT",
                    Type::Market => "MARKET",
                    Type::StopLoss => "STOP_LOSS",
                    Type::StopLossLimit => "STOP_LOSS_LIMIT",
                    Type::TakeProfit => "TAKE_PROFIT",
                    Type::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
                    Type::LimitMaker => "LIMIT_MAKER",
                }
            )
        }
    }

    ///  order quantity
    pub type Quantity = f32;

    ///  order price
    pub type Price = f32;

    ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
    pub type StopPrice = f32;

    ///  Used to uniquely identify this cancel. Automatically generated by default
    pub type NewClientOrderId = String;

    ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
    pub type IcebergQty = f32;

    ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum NewOrderRespType {
        #[serde(rename = "ACK")]
        #[allow(non_camel_case_types)]
        Ack,
        #[serde(rename = "RESULT")]
        #[allow(non_camel_case_types)]
        Result,
        #[serde(rename = "FULL")]
        #[allow(non_camel_case_types)]
        Full,
    }

    impl std::fmt::Display for NewOrderRespType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    NewOrderRespType::Ack => "ACK",
                    NewOrderRespType::Result => "RESULT",
                    NewOrderRespType::Full => "FULL",
                }
            )
        }
    }

    ///  default NO_SIDE_EFFECT

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum SideEffectType {
        #[serde(rename = "NO_SIDE_EFFECT")]
        #[allow(non_camel_case_types)]
        NoSideEffect,
        #[serde(rename = "MARGIN_BUY")]
        #[allow(non_camel_case_types)]
        MarginBuy,
        #[serde(rename = "AUTO_REPAY")]
        #[allow(non_camel_case_types)]
        AutoRepay,
    }

    impl std::fmt::Display for SideEffectType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    SideEffectType::NoSideEffect => "NO_SIDE_EFFECT",
                    SideEffectType::MarginBuy => "MARGIN_BUY",
                    SideEffectType::AutoRepay => "AUTO_REPAY",
                }
            )
        }
    }

    ///  order time in force

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum TimeInForce {
        #[serde(rename = "GTC")]
        #[allow(non_camel_case_types)]
        Gtc,
        #[serde(rename = "IOC")]
        #[allow(non_camel_case_types)]
        Ioc,
        #[serde(rename = "FOK")]
        #[allow(non_camel_case_types)]
        Fok,
    }

    impl std::fmt::Display for TimeInForce {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    TimeInForce::Gtc => "GTC",
                    TimeInForce::Ioc => "IOC",
                    TimeInForce::Fok => "FOK",
                }
            )
        }
    }

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_margin_order operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  SELL or BUY
        #[serde(rename = "side")]
        pub side: Side,
        ///  the order type
        #[serde(rename = "type")]
        pub type_: Type,
        ///  order quantity
        #[serde(rename = "quantity")]
        pub quantity: f32,
        ///  order price
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f32>,
        ///  Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
        #[serde(rename = "stop_price")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_price: Option<f32>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(rename = "new_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an iceberg order.
        #[serde(rename = "iceberg_qty")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iceberg_qty: Option<f32>,
        ///  Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types default to FULL, all other orders default to ACK.
        #[serde(rename = "new_order_resp_type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_order_resp_type: Option<NewOrderRespType>,
        ///  default NO_SIDE_EFFECT
        #[serde(rename = "side_effect_type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub side_effect_type: Option<SideEffectType>,
        ///  order time in force
        #[serde(rename = "time_in_force")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_in_force: Option<TimeInForce>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  margin pairs
    pub type Response200 = ();

    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  margin pairs
    pub type Status200 = ();

    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod delete_sapi_v1_margin_order {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for delete_sapi_v1_margin_order operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
        ///  order id
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                order_id: query.order_id,
                orig_client_order_id: query.orig_client_order_id,
                new_client_order_id: query.new_client_order_id,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                order_id: self.order_id.clone(),
                orig_client_order_id: self.orig_client_order_id.clone(),
                new_client_order_id: self.new_client_order_id.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  order id
    pub type OrderId = i64;

    ///  order id from client
    pub type OrigClientOrderId = String;

    ///  Used to uniquely identify this cancel. Automatically generated by default
    pub type NewClientOrderId = String;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for delete_sapi_v1_margin_order operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
        ///  order id
        #[serde(rename = "order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_id: Option<i64>,
        ///  order id from client
        #[serde(rename = "orig_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub orig_client_order_id: Option<String>,
        ///  Used to uniquely identify this cancel. Automatically generated by default
        #[serde(rename = "new_client_order_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_client_order_id: Option<String>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  cancelled margin order details
    pub type Response200 = components::schemas::MarginOrder;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  cancelled margin order details
    pub type Status200 = components::schemas::MarginOrder;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_pair {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_pair operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    /// Query parameters for get_sapi_v1_margin_pair operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  margin pair details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_buy_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_margin_trade: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_sell_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  margin pair details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_buy_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_margin_trade: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_sell_allowed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_price_index {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_price_index operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        pub symbol: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    /// Query parameters for get_sapi_v1_margin_price_index operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        pub symbol: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  Price index
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub calc_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  Price index
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub calc_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_repay {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_repay operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        pub asset: String,
        ///  the tranId in  `POST /sapi/v1/margin/repay`
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                tx_id: query.tx_id,
                start_time: query.start_time,
                end_time: query.end_time,
                current: query.current,
                size: query.size,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                tx_id: self.tx_id.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                current: self.current.clone(),
                size: self.size.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    ///  the tranId in  `POST /sapi/v1/margin/repay`
    pub type TxId = i64;

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Currently querying page. Start from 1. Default:1
    pub type Current = f64;

    ///  Default:10 Max:100
    pub type Size = f64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_repay operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        pub asset: String,
        ///  the tranId in  `POST /sapi/v1/margin/repay`
        #[serde(rename = "tx_id")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(rename = "current")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200RowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub principal: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<i64>,
    }

    ///  Load records
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Response200RowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200RowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interest: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub principal: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_id: Option<i64>,
    }

    ///  Load records
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Status200RowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_sapi_v1_margin_repay {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_margin_repay operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        pub asset: String,
        pub amount: f32,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                amount: query.amount,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                amount: self.amount.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    pub type Amount = f32;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_margin_repay operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        pub asset: String,

        #[serde(rename = "amount")]
        pub amount: f32,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  order details
    pub type Response200 = components::schemas::Transaction;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  order details
    pub type Status200 = components::schemas::Transaction;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_sapi_v1_margin_transfer {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_sapi_v1_margin_transfer operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        ///  Tranfer Type
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<Type>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                type_: query.type_,
                start_time: query.start_time,
                end_time: query.end_time,
                current: query.current,
                size: query.size,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                type_: self.type_.clone(),
                start_time: self.start_time.clone(),
                end_time: self.end_time.clone(),
                current: self.current.clone(),
                size: self.size.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    ///  Tranfer Type

    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
    pub enum Type {
        #[serde(rename = "ROLL_IN")]
        #[allow(non_camel_case_types)]
        RollIn,
        #[serde(rename = "ROLL_OUT")]
        #[allow(non_camel_case_types)]
        RollOut,
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Type::RollIn => "ROLL_IN",
                    Type::RollOut => "ROLL_OUT",
                }
            )
        }
    }

    ///  Timestamp in ms
    pub type StartTime = i64;

    ///  Timestamp in ms
    pub type EndTime = i64;

    ///  Currently querying page. Start from 1. Default:1
    pub type Current = f64;

    ///  Default:10 Max:100
    pub type Size = f64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_sapi_v1_margin_transfer operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset: Option<String>,
        ///  Tranfer Type
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<Type>,
        ///  Timestamp in ms
        #[serde(rename = "start_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<i64>,
        ///  Timestamp in ms
        #[serde(rename = "end_time")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<i64>,
        ///  Currently querying page. Start from 1. Default:1
        #[serde(rename = "current")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub current: Option<f64>,
        ///  Default:10 Max:100
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Unspecified(T),
    }

    ///  margin account transfer history, response in descending order

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  margin account transfer history, response in descending order

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
}

pub mod post_sapi_v1_margin_transfer {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_margin_transfer operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  The asset being transferred, e.g., BTC
        pub asset: String,
        pub amount: f32,
        ///  1 -> transfer from main account to margin account \
        ///  2 -> transfer from margin account to main account
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                asset: query.asset,
                amount: query.amount,
                type_: query.type_,
                recv_window: query.recv_window,
                timestamp: query.timestamp,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                asset: self.asset.clone(),
                amount: self.amount.clone(),
                type_: self.type_.clone(),
                recv_window: self.recv_window.clone(),
                timestamp: self.timestamp.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  The asset being transferred, e.g., BTC
    pub type Asset = String;

    pub type Amount = f32;

    ///  1 -> transfer from main account to margin account \
    ///  2 -> transfer from margin account to main account
    pub type Type = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for post_sapi_v1_margin_transfer operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  The asset being transferred, e.g., BTC
        #[serde(rename = "asset")]
        pub asset: String,

        #[serde(rename = "amount")]
        pub amount: f32,
        ///  1 -> transfer from main account to margin account \
        ///  2 -> transfer from margin account to main account
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<i64>,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  transfer Id
    pub type Response200 = components::schemas::Transaction;
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  transfer Id
    pub type Status200 = components::schemas::Transaction;
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod post_sapi_v1_user_data_stream {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for post_sapi_v1_user_data_stream operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response401(Response401),
        Unspecified(T),
    }

    ///  listen key
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  listen key
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod put_sapi_v1_user_data_stream {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for put_sapi_v1_user_data_stream operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  User websocket listen key
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                listen_key: query.listen_key,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                listen_key: self.listen_key.clone(),
            }
        }
    }
    ///  User websocket listen key
    pub type ListenKey = String;

    /// Query parameters for put_sapi_v1_user_data_stream operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  User websocket listen key
        #[serde(rename = "listen_key")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  empty object

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  empty object

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod delete_sapi_v1_user_data_stream {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for delete_sapi_v1_user_data_stream operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  User websocket listen key
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                listen_key: query.listen_key,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                listen_key: self.listen_key.clone(),
            }
        }
    }
    ///  User websocket listen key
    pub type ListenKey = String;

    /// Query parameters for delete_sapi_v1_user_data_stream operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  User websocket listen key
        #[serde(rename = "listen_key")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen_key: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct Body;

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  empty object

    pub type Response200 = ();
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  empty object

    pub type Status200 = ();
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_wapi_v3_account_status_html {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_wapi_v3_account_status_html operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_wapi_v3_account_status_html operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    ///  account status
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub msg: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  account status
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub msg: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_wapi_v3_api_trading_status_html {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_wapi_v3_api_trading_status_html operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_wapi_v3_api_trading_status_html operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200StatusIndicatorsBtcusdtItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub c: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub i: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub t: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub v: Option<f32>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200StatusIndicators {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub btcusdt: Option<Vec<Response200StatusIndicatorsBtcusdtItem>>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200StatusTriggerCondition {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gcr: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ifer: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ufr: Option<i64>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200Status {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub indicators: Option<Response200StatusIndicators>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub planned_recover_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_condition: Option<Response200StatusTriggerCondition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub update_time: Option<i64>,
    }

    ///  API trading staus details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<Response200Status>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200StatusIndicatorsBtcusdtItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub c: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub i: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub t: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub v: Option<f32>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200StatusIndicators {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub btcusdt: Option<Vec<Status200StatusIndicatorsBtcusdtItem>>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200StatusTriggerCondition {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gcr: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ifer: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ufr: Option<i64>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200Status {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub indicators: Option<Status200StatusIndicators>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_locked: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub planned_recover_time: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_condition: Option<Status200StatusTriggerCondition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub update_time: Option<i64>,
    }

    ///  API trading staus details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<Status200Status>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_wapi_v3_asset_detail_html {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_wapi_v3_asset_detail_html operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_wapi_v3_asset_detail_html operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200AssetDetailCtr {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_status: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_withdraw_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_fee: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_status: Option<bool>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200AssetDetail {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ctr: Option<Response200AssetDetailCtr>,
    }

    ///  asset details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_detail: Option<Response200AssetDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200AssetDetailCtr {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deposit_status: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_withdraw_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_fee: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub withdraw_status: Option<bool>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200AssetDetail {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ctr: Option<Status200AssetDetailCtr>,
    }

    ///  asset details
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub asset_detail: Option<Status200AssetDetail>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_wapi_v3_system_status_html {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_wapi_v3_system_status_html operation
    pub struct Parameters;

    impl Parameters {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Unspecified(T),
    }

    ///  system status

    pub type Response200 = ();

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    ///  system status

    pub type Status200 = ();
}

pub mod get_wapi_v3_trade_fee_html {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_wapi_v3_trade_fee_html operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                symbol: query.symbol,
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                symbol: self.symbol.clone(),
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  trading symbol, e.g. BNBUSDT
    pub type Symbol = String;

    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_wapi_v3_trade_fee_html operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  trading symbol, e.g. BNBUSDT
        #[serde(rename = "symbol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200TradeFeeItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maker: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub taker: Option<i64>,
    }

    ///  Trading fee
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trade_fee: Option<Vec<Response200TradeFeeItem>>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200TradeFeeItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maker: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub symbol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub taker: Option<i64>,
    }

    ///  Trading fee
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trade_fee: Option<Vec<Status200TradeFeeItem>>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}

pub mod get_wapi_v3_user_asset_dribblet_log_html {
    use super::components;
    use serde::{Deserialize, Serialize};

    /// Parameters for get_wapi_v3_user_asset_dribblet_log_html operation
    #[derive(Deserialize, Debug)]
    pub struct Parameters {
        ///  UTC timestamp
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        pub signature: String,
    }

    impl Parameters {
        pub fn new(query: Query) -> Self {
            Self {
                timestamp: query.timestamp,
                recv_window: query.recv_window,
                signature: query.signature,
            }
        }

        pub fn query(&self) -> Query {
            Query {
                timestamp: self.timestamp.clone(),
                recv_window: self.recv_window.clone(),
                signature: self.signature.clone(),
            }
        }
    }
    ///  UTC timestamp
    pub type Timestamp = i64;

    ///  The value cannot be greater than 60000
    pub type RecvWindow = i64;

    ///  signature
    pub type Signature = String;

    /// Query parameters for get_wapi_v3_user_asset_dribblet_log_html operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        ///  UTC timestamp
        #[serde(rename = "timestamp")]
        pub timestamp: i64,
        ///  The value cannot be greater than 60000
        #[serde(rename = "recv_window")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recv_window: Option<i64>,
        ///  signature
        #[serde(rename = "signature")]
        pub signature: String,
    }

    #[derive(Debug)]
    pub enum Response<T> {
        Response200(Response200),
        Response400(Response400),
        Response401(Response401),
        Unspecified(T),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200ResultsRowsItemLogsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operate_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_charge_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfered_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uid: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200ResultsRowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub logs: Option<Vec<Response200ResultsRowsItemLogsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operate_item: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_charge_total: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfered_total: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200Results {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Response200ResultsRowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }

    ///  Dust log records
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Response200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Response200Results>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Response400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Response401 = components::schemas::Error;

    #[derive(Debug)]
    pub enum Success {
        Status200(Status200),
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
        Status400(Status400),
        Status401(Status401),
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Status400(status) => write!(f, "status 400: {:?}", status),
                Self::Status401(status) => write!(f, "status 401: {:?}", status),
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200ResultsRowsItemLogsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_asset: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operate_time: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_charge_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfered_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uid: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200ResultsRowsItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub logs: Option<Vec<Status200ResultsRowsItemLogsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operate_item: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_charge_total: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tran_id: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transfered_total: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200Results {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rows: Option<Vec<Status200ResultsRowsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }

    ///  Dust log records
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Status200 {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Status200Results>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success: Option<bool>,
    }
    ///  Bad Request
    pub type Status400 = components::schemas::Error;
    ///  Unauthorized Request
    pub type Status401 = components::schemas::Error;
}
