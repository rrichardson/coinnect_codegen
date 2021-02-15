use actix_web::http::Method;
use maplit::hashmap;
use maplit::hashset;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

pub static SECURITY_MATRIX: Lazy<HashMap<(&str, Method), HashSet<&str>>> = Lazy::new(|| {
    hashmap! {
        ("/api/v3/account", Method::GET) => hashset![
        ],
        ("/api/v3/aggTrades", Method::GET) => hashset![
        ],
        ("/api/v3/allOrderList", Method::GET) => hashset![
        ],
        ("/api/v3/allOrders", Method::GET) => hashset![
        ],
        ("/api/v3/avgPrice", Method::GET) => hashset![
        ],
        ("/api/v3/depth", Method::GET) => hashset![
        ],
        ("/api/v3/exchangeInfo", Method::GET) => hashset![
        ],
        ("/api/v3/historicalTrades", Method::GET) => hashset![
        ],
        ("/api/v3/klines", Method::GET) => hashset![
        ],
        ("/api/v3/myTrades", Method::GET) => hashset![
        ],
        ("/api/v3/openOrderList", Method::GET) => hashset![
        ],
        ("/api/v3/openOrders", Method::GET) => hashset![
        ],
        ("/api/v3/openOrders", Method::DELETE) => hashset![
        ],
        ("/api/v3/order", Method::GET) => hashset![
        ],
        ("/api/v3/order", Method::POST) => hashset![
        ],
        ("/api/v3/order", Method::DELETE) => hashset![
        ],
        ("/api/v3/order/oco", Method::POST) => hashset![
        ],
        ("/api/v3/order/test", Method::POST) => hashset![
        ],
        ("/api/v3/orderList", Method::GET) => hashset![
        ],
        ("/api/v3/orderList", Method::DELETE) => hashset![
        ],
        ("/api/v3/ping", Method::GET) => hashset![
        ],
        ("/api/v3/ticker/24hr", Method::GET) => hashset![
        ],
        ("/api/v3/ticker/bookTicker", Method::GET) => hashset![
        ],
        ("/api/v3/ticker/price", Method::GET) => hashset![
        ],
        ("/api/v3/time", Method::GET) => hashset![
        ],
        ("/api/v3/trades", Method::GET) => hashset![
        ],
        ("/api/v3/userDataStream", Method::POST) => hashset![
        ],
        ("/api/v3/userDataStream", Method::PUT) => hashset![
        ],
        ("/api/v3/userDataStream", Method::DELETE) => hashset![
        ],
        ("/sapi/v1/account/disableFastWithdrawSwitch", Method::POST) => hashset![
        ],
        ("/sapi/v1/account/enableFastWithdrawSwitch", Method::POST) => hashset![
        ],
        ("/sapi/v1/accountSnapshot", Method::GET) => hashset![
        ],
        ("/sapi/v1/asset/assetDividend", Method::GET) => hashset![
        ],
        ("/sapi/v1/asset/dust", Method::POST) => hashset![
        ],
        ("/sapi/v1/capital/config/getall", Method::GET) => hashset![
        ],
        ("/sapi/v1/capital/deposit/address", Method::GET) => hashset![
        ],
        ("/sapi/v1/capital/deposit/hisrec", Method::GET) => hashset![
        ],
        ("/sapi/v1/capital/withdraw/apply", Method::POST) => hashset![
        ],
        ("/sapi/v1/capital/withdraw/history", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/account", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/allAssets", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/allOrders", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/allPairs", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/asset", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/forceLiquidationRec", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/interestHistory", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/loan", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/loan", Method::POST) => hashset![
        ],
        ("/sapi/v1/margin/maxBorrowable", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/maxTransferable", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/myTrades", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/openOrders", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/order", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/order", Method::POST) => hashset![
        ],
        ("/sapi/v1/margin/order", Method::DELETE) => hashset![
        ],
        ("/sapi/v1/margin/pair", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/priceIndex", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/repay", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/repay", Method::POST) => hashset![
        ],
        ("/sapi/v1/margin/transfer", Method::GET) => hashset![
        ],
        ("/sapi/v1/margin/transfer", Method::POST) => hashset![
        ],
        ("/sapi/v1/userDataStream", Method::POST) => hashset![
        ],
        ("/sapi/v1/userDataStream", Method::PUT) => hashset![
        ],
        ("/sapi/v1/userDataStream", Method::DELETE) => hashset![
        ],
        ("/wapi/v3/accountStatus.html", Method::GET) => hashset![
        ],
        ("/wapi/v3/apiTradingStatus.html", Method::GET) => hashset![
        ],
        ("/wapi/v3/assetDetail.html", Method::GET) => hashset![
        ],
        ("/wapi/v3/systemStatus.html", Method::GET) => hashset![
        ],
        ("/wapi/v3/tradeFee.html", Method::GET) => hashset![
        ],
        ("/wapi/v3/userAssetDribbletLog.html", Method::GET) => hashset![
        ],
    }
});
