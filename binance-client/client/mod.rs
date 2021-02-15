#![allow(clippy::ptr_arg)]
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/* Reqwest's errors are bad-mannered and recurse on their source when displayed.
 * This behavior doesn't interact well with thiserror which also recurse on error's cause
 * when displayed. To prevent this issue, this wrapper hides the error's source from thiserror.
 */
pub struct ReqwestError {
    err: reqwest::Error,
}

impl ReqwestError {
    pub fn new(err: reqwest::Error) -> Self {
        Self { err }
    }
}

impl std::error::Error for ReqwestError {}

impl From<reqwest::Error> for ReqwestError {
    fn from(err: reqwest::Error) -> Self {
        Self::new(err)
    }
}

impl std::fmt::Debug for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(status) = self.err.status() {
            write!(f, "{:?}: {:?}", self.err, status)
        } else {
            std::fmt::Debug::fmt(&self.err, f)
        }
    }
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(status) = self.err.status() {
            write!(f, "{}: {}", self.err, status)
        } else {
            std::fmt::Display::fmt(&self.err, f)
        }
    }
}

#[derive(Clone)]
pub struct BinanceSpotPublicApiClient {
    pub url: Url,
    pub client: reqwest::Client,
}

impl BinanceSpotPublicApiClient {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).expect("cannot parse url");
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("bad client build");
        self
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_account(
        &self,
        parameters: &get_api_v3_account::Parameters,
    ) -> Result<get_api_v3_account::Success, get_api_v3_account::Error> {
        use get_api_v3_account::*;
        let url = self
            .url
            .join("/api/v3/account".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_agg_trades(
        &self,
        parameters: &get_api_v3_agg_trades::Parameters,
    ) -> Result<get_api_v3_agg_trades::Success, get_api_v3_agg_trades::Error> {
        use get_api_v3_agg_trades::*;
        let url = self
            .url
            .join("/api/v3/aggTrades".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_all_order_list(
        &self,
        parameters: &get_api_v3_all_order_list::Parameters,
    ) -> Result<get_api_v3_all_order_list::Success, get_api_v3_all_order_list::Error> {
        use get_api_v3_all_order_list::*;
        let url = self
            .url
            .join("/api/v3/allOrderList".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_all_orders(
        &self,
        parameters: &get_api_v3_all_orders::Parameters,
    ) -> Result<get_api_v3_all_orders::Success, get_api_v3_all_orders::Error> {
        use get_api_v3_all_orders::*;
        let url = self
            .url
            .join("/api/v3/allOrders".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_avg_price(
        &self,
        parameters: &get_api_v3_avg_price::Parameters,
    ) -> Result<get_api_v3_avg_price::Success, get_api_v3_avg_price::Error> {
        use get_api_v3_avg_price::*;
        let url = self
            .url
            .join("/api/v3/avgPrice".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_depth(
        &self,
        parameters: &get_api_v3_depth::Parameters,
    ) -> Result<get_api_v3_depth::Success, get_api_v3_depth::Error> {
        use get_api_v3_depth::*;
        let url = self
            .url
            .join("/api/v3/depth".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_exchange_info(
        &self,
    ) -> Result<get_api_v3_exchange_info::Success, get_api_v3_exchange_info::Error> {
        use get_api_v3_exchange_info::*;
        let url = self
            .url
            .join("/api/v3/exchangeInfo".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_historical_trades(
        &self,
        parameters: &get_api_v3_historical_trades::Parameters,
    ) -> Result<get_api_v3_historical_trades::Success, get_api_v3_historical_trades::Error> {
        use get_api_v3_historical_trades::*;
        let url = self
            .url
            .join("/api/v3/historicalTrades".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_klines(
        &self,
        parameters: &get_api_v3_klines::Parameters,
    ) -> Result<get_api_v3_klines::Success, get_api_v3_klines::Error> {
        use get_api_v3_klines::*;
        let url = self
            .url
            .join("/api/v3/klines".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_my_trades(
        &self,
        parameters: &get_api_v3_my_trades::Parameters,
    ) -> Result<get_api_v3_my_trades::Success, get_api_v3_my_trades::Error> {
        use get_api_v3_my_trades::*;
        let url = self
            .url
            .join("/api/v3/myTrades".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_open_order_list(
        &self,
        parameters: &get_api_v3_open_order_list::Parameters,
    ) -> Result<get_api_v3_open_order_list::Success, get_api_v3_open_order_list::Error> {
        use get_api_v3_open_order_list::*;
        let url = self
            .url
            .join("/api/v3/openOrderList".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_open_orders(
        &self,
        parameters: &get_api_v3_open_orders::Parameters,
    ) -> Result<get_api_v3_open_orders::Success, get_api_v3_open_orders::Error> {
        use get_api_v3_open_orders::*;
        let url = self
            .url
            .join("/api/v3/openOrders".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn delete_api_v3_open_orders(
        &self,
        parameters: &delete_api_v3_open_orders::Parameters,
    ) -> Result<delete_api_v3_open_orders::Success, delete_api_v3_open_orders::Error> {
        use delete_api_v3_open_orders::*;
        let url = self
            .url
            .join("/api/v3/openOrders".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .delete(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_order(
        &self,
        parameters: &get_api_v3_order::Parameters,
    ) -> Result<get_api_v3_order::Success, get_api_v3_order::Error> {
        use get_api_v3_order::*;
        let url = self
            .url
            .join("/api/v3/order".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_api_v3_order(
        &self,
        parameters: &post_api_v3_order::Parameters,
    ) -> Result<post_api_v3_order::Success, post_api_v3_order::Error> {
        use post_api_v3_order::*;
        let url = self
            .url
            .join("/api/v3/order".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn delete_api_v3_order(
        &self,
        parameters: &delete_api_v3_order::Parameters,
    ) -> Result<delete_api_v3_order::Success, delete_api_v3_order::Error> {
        use delete_api_v3_order::*;
        let url = self
            .url
            .join("/api/v3/order".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .delete(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_api_v3_order_oco(
        &self,
        parameters: &post_api_v3_order_oco::Parameters,
    ) -> Result<post_api_v3_order_oco::Success, post_api_v3_order_oco::Error> {
        use post_api_v3_order_oco::*;
        let url = self
            .url
            .join("/api/v3/order/oco".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_api_v3_order_test(
        &self,
        parameters: &post_api_v3_order_test::Parameters,
    ) -> Result<post_api_v3_order_test::Success, post_api_v3_order_test::Error> {
        use post_api_v3_order_test::*;
        let url = self
            .url
            .join("/api/v3/order/test".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = ();
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_order_list(
        &self,
        parameters: &get_api_v3_order_list::Parameters,
    ) -> Result<get_api_v3_order_list::Success, get_api_v3_order_list::Error> {
        use get_api_v3_order_list::*;
        let url = self
            .url
            .join("/api/v3/orderList".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn delete_api_v3_order_list(
        &self,
        parameters: &delete_api_v3_order_list::Parameters,
    ) -> Result<delete_api_v3_order_list::Success, delete_api_v3_order_list::Error> {
        use delete_api_v3_order_list::*;
        let url = self
            .url
            .join("/api/v3/orderList".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .delete(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_ping(
        &self,
    ) -> Result<get_api_v3_ping::Success, get_api_v3_ping::Error> {
        use get_api_v3_ping::*;
        let url = self
            .url
            .join("/api/v3/ping".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_ticker_24hr(
        &self,
        parameters: &get_api_v3_ticker_24hr::Parameters,
    ) -> Result<get_api_v3_ticker_24hr::Success, get_api_v3_ticker_24hr::Error> {
        use get_api_v3_ticker_24hr::*;
        let url = self
            .url
            .join("/api/v3/ticker/24hr".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_ticker_book_ticker(
        &self,
        parameters: &get_api_v3_ticker_book_ticker::Parameters,
    ) -> Result<get_api_v3_ticker_book_ticker::Success, get_api_v3_ticker_book_ticker::Error> {
        use get_api_v3_ticker_book_ticker::*;
        let url = self
            .url
            .join("/api/v3/ticker/bookTicker".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_ticker_price(
        &self,
        parameters: &get_api_v3_ticker_price::Parameters,
    ) -> Result<get_api_v3_ticker_price::Success, get_api_v3_ticker_price::Error> {
        use get_api_v3_ticker_price::*;
        let url = self
            .url
            .join("/api/v3/ticker/price".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_time(
        &self,
    ) -> Result<get_api_v3_time::Success, get_api_v3_time::Error> {
        use get_api_v3_time::*;
        let url = self
            .url
            .join("/api/v3/time".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_api_v3_trades(
        &self,
        parameters: &get_api_v3_trades::Parameters,
    ) -> Result<get_api_v3_trades::Success, get_api_v3_trades::Error> {
        use get_api_v3_trades::*;
        let url = self
            .url
            .join("/api/v3/trades".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_api_v3_user_data_stream(
        &self,
    ) -> Result<post_api_v3_user_data_stream::Success, post_api_v3_user_data_stream::Error> {
        use post_api_v3_user_data_stream::*;
        let url = self
            .url
            .join("/api/v3/userDataStream".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn put_api_v3_user_data_stream(
        &self,
        parameters: &put_api_v3_user_data_stream::Parameters,
    ) -> Result<put_api_v3_user_data_stream::Success, put_api_v3_user_data_stream::Error> {
        use put_api_v3_user_data_stream::*;
        let url = self
            .url
            .join("/api/v3/userDataStream".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .put(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn delete_api_v3_user_data_stream(
        &self,
        parameters: &delete_api_v3_user_data_stream::Parameters,
    ) -> Result<delete_api_v3_user_data_stream::Success, delete_api_v3_user_data_stream::Error>
    {
        use delete_api_v3_user_data_stream::*;
        let url = self
            .url
            .join("/api/v3/userDataStream".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .delete(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_account_disable_fast_withdraw_switch(
        &self,
        parameters: &post_sapi_v1_account_disable_fast_withdraw_switch::Parameters,
    ) -> Result<
        post_sapi_v1_account_disable_fast_withdraw_switch::Success,
        post_sapi_v1_account_disable_fast_withdraw_switch::Error,
    > {
        use post_sapi_v1_account_disable_fast_withdraw_switch::*;
        let url = self
            .url
            .join("/sapi/v1/account/disableFastWithdrawSwitch".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = ();
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_account_enable_fast_withdraw_switch(
        &self,
        parameters: &post_sapi_v1_account_enable_fast_withdraw_switch::Parameters,
    ) -> Result<
        post_sapi_v1_account_enable_fast_withdraw_switch::Success,
        post_sapi_v1_account_enable_fast_withdraw_switch::Error,
    > {
        use post_sapi_v1_account_enable_fast_withdraw_switch::*;
        let url = self
            .url
            .join("/sapi/v1/account/enableFastWithdrawSwitch".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = ();
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_account_snapshot(
        &self,
        parameters: &get_sapi_v1_account_snapshot::Parameters,
    ) -> Result<get_sapi_v1_account_snapshot::Success, get_sapi_v1_account_snapshot::Error> {
        use get_sapi_v1_account_snapshot::*;
        let url = self
            .url
            .join("/sapi/v1/accountSnapshot".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_asset_asset_dividend(
        &self,
        parameters: &get_sapi_v1_asset_asset_dividend::Parameters,
    ) -> Result<get_sapi_v1_asset_asset_dividend::Success, get_sapi_v1_asset_asset_dividend::Error>
    {
        use get_sapi_v1_asset_asset_dividend::*;
        let url = self
            .url
            .join("/sapi/v1/asset/assetDividend".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_asset_dust(
        &self,
        parameters: &post_sapi_v1_asset_dust::Parameters,
    ) -> Result<post_sapi_v1_asset_dust::Success, post_sapi_v1_asset_dust::Error> {
        use post_sapi_v1_asset_dust::*;
        let url = self
            .url
            .join("/sapi/v1/asset/dust".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_capital_config_getall(
        &self,
        parameters: &get_sapi_v1_capital_config_getall::Parameters,
    ) -> Result<get_sapi_v1_capital_config_getall::Success, get_sapi_v1_capital_config_getall::Error>
    {
        use get_sapi_v1_capital_config_getall::*;
        let url = self
            .url
            .join("/sapi/v1/capital/config/getall".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_capital_deposit_address(
        &self,
        parameters: &get_sapi_v1_capital_deposit_address::Parameters,
    ) -> Result<
        get_sapi_v1_capital_deposit_address::Success,
        get_sapi_v1_capital_deposit_address::Error,
    > {
        use get_sapi_v1_capital_deposit_address::*;
        let url = self
            .url
            .join("/sapi/v1/capital/deposit/address".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_capital_deposit_hisrec(
        &self,
        parameters: &get_sapi_v1_capital_deposit_hisrec::Parameters,
    ) -> Result<
        get_sapi_v1_capital_deposit_hisrec::Success,
        get_sapi_v1_capital_deposit_hisrec::Error,
    > {
        use get_sapi_v1_capital_deposit_hisrec::*;
        let url = self
            .url
            .join("/sapi/v1/capital/deposit/hisrec".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_capital_withdraw_apply(
        &self,
        parameters: &post_sapi_v1_capital_withdraw_apply::Parameters,
    ) -> Result<
        post_sapi_v1_capital_withdraw_apply::Success,
        post_sapi_v1_capital_withdraw_apply::Error,
    > {
        use post_sapi_v1_capital_withdraw_apply::*;
        let url = self
            .url
            .join("/sapi/v1/capital/withdraw/apply".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_capital_withdraw_history(
        &self,
        parameters: &get_sapi_v1_capital_withdraw_history::Parameters,
    ) -> Result<
        get_sapi_v1_capital_withdraw_history::Success,
        get_sapi_v1_capital_withdraw_history::Error,
    > {
        use get_sapi_v1_capital_withdraw_history::*;
        let url = self
            .url
            .join("/sapi/v1/capital/withdraw/history".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_account(
        &self,
        parameters: &get_sapi_v1_margin_account::Parameters,
    ) -> Result<get_sapi_v1_margin_account::Success, get_sapi_v1_margin_account::Error> {
        use get_sapi_v1_margin_account::*;
        let url = self
            .url
            .join("/sapi/v1/margin/account".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_all_assets(
        &self,
    ) -> Result<get_sapi_v1_margin_all_assets::Success, get_sapi_v1_margin_all_assets::Error> {
        use get_sapi_v1_margin_all_assets::*;
        let url = self
            .url
            .join("/sapi/v1/margin/allAssets".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_all_orders(
        &self,
        parameters: &get_sapi_v1_margin_all_orders::Parameters,
    ) -> Result<get_sapi_v1_margin_all_orders::Success, get_sapi_v1_margin_all_orders::Error> {
        use get_sapi_v1_margin_all_orders::*;
        let url = self
            .url
            .join("/sapi/v1/margin/allOrders".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_all_pairs(
        &self,
    ) -> Result<get_sapi_v1_margin_all_pairs::Success, get_sapi_v1_margin_all_pairs::Error> {
        use get_sapi_v1_margin_all_pairs::*;
        let url = self
            .url
            .join("/sapi/v1/margin/allPairs".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_asset(
        &self,
        parameters: &get_sapi_v1_margin_asset::Parameters,
    ) -> Result<get_sapi_v1_margin_asset::Success, get_sapi_v1_margin_asset::Error> {
        use get_sapi_v1_margin_asset::*;
        let url = self
            .url
            .join("/sapi/v1/margin/asset".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_force_liquidation_rec(
        &self,
        parameters: &get_sapi_v1_margin_force_liquidation_rec::Parameters,
    ) -> Result<
        get_sapi_v1_margin_force_liquidation_rec::Success,
        get_sapi_v1_margin_force_liquidation_rec::Error,
    > {
        use get_sapi_v1_margin_force_liquidation_rec::*;
        let url = self
            .url
            .join("/sapi/v1/margin/forceLiquidationRec".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_interest_history(
        &self,
        parameters: &get_sapi_v1_margin_interest_history::Parameters,
    ) -> Result<
        get_sapi_v1_margin_interest_history::Success,
        get_sapi_v1_margin_interest_history::Error,
    > {
        use get_sapi_v1_margin_interest_history::*;
        let url = self
            .url
            .join("/sapi/v1/margin/interestHistory".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_loan(
        &self,
        parameters: &get_sapi_v1_margin_loan::Parameters,
    ) -> Result<get_sapi_v1_margin_loan::Success, get_sapi_v1_margin_loan::Error> {
        use get_sapi_v1_margin_loan::*;
        let url = self
            .url
            .join("/sapi/v1/margin/loan".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_margin_loan(
        &self,
        parameters: &post_sapi_v1_margin_loan::Parameters,
    ) -> Result<post_sapi_v1_margin_loan::Success, post_sapi_v1_margin_loan::Error> {
        use post_sapi_v1_margin_loan::*;
        let url = self
            .url
            .join("/sapi/v1/margin/loan".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_max_borrowable(
        &self,
        parameters: &get_sapi_v1_margin_max_borrowable::Parameters,
    ) -> Result<get_sapi_v1_margin_max_borrowable::Success, get_sapi_v1_margin_max_borrowable::Error>
    {
        use get_sapi_v1_margin_max_borrowable::*;
        let url = self
            .url
            .join("/sapi/v1/margin/maxBorrowable".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_max_transferable(
        &self,
        parameters: &get_sapi_v1_margin_max_transferable::Parameters,
    ) -> Result<
        get_sapi_v1_margin_max_transferable::Success,
        get_sapi_v1_margin_max_transferable::Error,
    > {
        use get_sapi_v1_margin_max_transferable::*;
        let url = self
            .url
            .join("/sapi/v1/margin/maxTransferable".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_my_trades(
        &self,
        parameters: &get_sapi_v1_margin_my_trades::Parameters,
    ) -> Result<get_sapi_v1_margin_my_trades::Success, get_sapi_v1_margin_my_trades::Error> {
        use get_sapi_v1_margin_my_trades::*;
        let url = self
            .url
            .join("/sapi/v1/margin/myTrades".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_open_orders(
        &self,
        parameters: &get_sapi_v1_margin_open_orders::Parameters,
    ) -> Result<get_sapi_v1_margin_open_orders::Success, get_sapi_v1_margin_open_orders::Error>
    {
        use get_sapi_v1_margin_open_orders::*;
        let url = self
            .url
            .join("/sapi/v1/margin/openOrders".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_order(
        &self,
        parameters: &get_sapi_v1_margin_order::Parameters,
    ) -> Result<get_sapi_v1_margin_order::Success, get_sapi_v1_margin_order::Error> {
        use get_sapi_v1_margin_order::*;
        let url = self
            .url
            .join("/sapi/v1/margin/order".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_margin_order(
        &self,
        parameters: &post_sapi_v1_margin_order::Parameters,
    ) -> Result<post_sapi_v1_margin_order::Success, post_sapi_v1_margin_order::Error> {
        use post_sapi_v1_margin_order::*;
        let url = self
            .url
            .join("/sapi/v1/margin/order".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn delete_sapi_v1_margin_order(
        &self,
        parameters: &delete_sapi_v1_margin_order::Parameters,
    ) -> Result<delete_sapi_v1_margin_order::Success, delete_sapi_v1_margin_order::Error> {
        use delete_sapi_v1_margin_order::*;
        let url = self
            .url
            .join("/sapi/v1/margin/order".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .delete(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_pair(
        &self,
        parameters: &get_sapi_v1_margin_pair::Parameters,
    ) -> Result<get_sapi_v1_margin_pair::Success, get_sapi_v1_margin_pair::Error> {
        use get_sapi_v1_margin_pair::*;
        let url = self
            .url
            .join("/sapi/v1/margin/pair".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_price_index(
        &self,
        parameters: &get_sapi_v1_margin_price_index::Parameters,
    ) -> Result<get_sapi_v1_margin_price_index::Success, get_sapi_v1_margin_price_index::Error>
    {
        use get_sapi_v1_margin_price_index::*;
        let url = self
            .url
            .join("/sapi/v1/margin/priceIndex".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_repay(
        &self,
        parameters: &get_sapi_v1_margin_repay::Parameters,
    ) -> Result<get_sapi_v1_margin_repay::Success, get_sapi_v1_margin_repay::Error> {
        use get_sapi_v1_margin_repay::*;
        let url = self
            .url
            .join("/sapi/v1/margin/repay".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_margin_repay(
        &self,
        parameters: &post_sapi_v1_margin_repay::Parameters,
    ) -> Result<post_sapi_v1_margin_repay::Success, post_sapi_v1_margin_repay::Error> {
        use post_sapi_v1_margin_repay::*;
        let url = self
            .url
            .join("/sapi/v1/margin/repay".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_sapi_v1_margin_transfer(
        &self,
        parameters: &get_sapi_v1_margin_transfer::Parameters,
    ) -> Result<get_sapi_v1_margin_transfer::Success, get_sapi_v1_margin_transfer::Error> {
        use get_sapi_v1_margin_transfer::*;
        let url = self
            .url
            .join("/sapi/v1/margin/transfer".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_margin_transfer(
        &self,
        parameters: &post_sapi_v1_margin_transfer::Parameters,
    ) -> Result<post_sapi_v1_margin_transfer::Success, post_sapi_v1_margin_transfer::Error> {
        use post_sapi_v1_margin_transfer::*;
        let url = self
            .url
            .join("/sapi/v1/margin/transfer".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn post_sapi_v1_user_data_stream(
        &self,
    ) -> Result<post_sapi_v1_user_data_stream::Success, post_sapi_v1_user_data_stream::Error> {
        use post_sapi_v1_user_data_stream::*;
        let url = self
            .url
            .join("/sapi/v1/userDataStream".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .post(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn put_sapi_v1_user_data_stream(
        &self,
        parameters: &put_sapi_v1_user_data_stream::Parameters,
    ) -> Result<put_sapi_v1_user_data_stream::Success, put_sapi_v1_user_data_stream::Error> {
        use put_sapi_v1_user_data_stream::*;
        let url = self
            .url
            .join("/sapi/v1/userDataStream".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .put(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn delete_sapi_v1_user_data_stream(
        &self,
        parameters: &delete_sapi_v1_user_data_stream::Parameters,
    ) -> Result<delete_sapi_v1_user_data_stream::Success, delete_sapi_v1_user_data_stream::Error>
    {
        use delete_sapi_v1_user_data_stream::*;
        let url = self
            .url
            .join("/sapi/v1/userDataStream".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .delete(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_wapi_v3_account_status_html(
        &self,
        parameters: &get_wapi_v3_account_status_html::Parameters,
    ) -> Result<get_wapi_v3_account_status_html::Success, get_wapi_v3_account_status_html::Error>
    {
        use get_wapi_v3_account_status_html::*;
        let url = self
            .url
            .join("/wapi/v3/accountStatus.html".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_wapi_v3_api_trading_status_html(
        &self,
        parameters: &get_wapi_v3_api_trading_status_html::Parameters,
    ) -> Result<
        get_wapi_v3_api_trading_status_html::Success,
        get_wapi_v3_api_trading_status_html::Error,
    > {
        use get_wapi_v3_api_trading_status_html::*;
        let url = self
            .url
            .join("/wapi/v3/apiTradingStatus.html".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_wapi_v3_asset_detail_html(
        &self,
        parameters: &get_wapi_v3_asset_detail_html::Parameters,
    ) -> Result<get_wapi_v3_asset_detail_html::Success, get_wapi_v3_asset_detail_html::Error> {
        use get_wapi_v3_asset_detail_html::*;
        let url = self
            .url
            .join("/wapi/v3/assetDetail.html".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_wapi_v3_system_status_html(
        &self,
    ) -> Result<get_wapi_v3_system_status_html::Success, get_wapi_v3_system_status_html::Error>
    {
        use get_wapi_v3_system_status_html::*;
        let url = self
            .url
            .join("/wapi/v3/systemStatus.html".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_wapi_v3_trade_fee_html(
        &self,
        parameters: &get_wapi_v3_trade_fee_html::Parameters,
    ) -> Result<get_wapi_v3_trade_fee_html::Success, get_wapi_v3_trade_fee_html::Error> {
        use get_wapi_v3_trade_fee_html::*;
        let url = self
            .url
            .join("/wapi/v3/tradeFee.html".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }

    #[allow(clippy::unit_arg)]
    pub async fn get_wapi_v3_user_asset_dribblet_log_html(
        &self,
        parameters: &get_wapi_v3_user_asset_dribblet_log_html::Parameters,
    ) -> Result<
        get_wapi_v3_user_asset_dribblet_log_html::Success,
        get_wapi_v3_user_asset_dribblet_log_html::Error,
    > {
        use get_wapi_v3_user_asset_dribblet_log_html::*;
        let url = self
            .url
            .join("/wapi/v3/userAssetDribbletLog.html".trim_start_matches('/'))
            .expect("url parse error");
        let response = self
            .client
            .get(url)
            .query(&parameters.query())
            .send()
            .await
            .map_err(ReqwestError::new)?;
        match response.status().as_str() {
            "200" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Ok(Success::Status200(response_body))
            }
            "400" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status400(response_body))
            }
            "401" => {
                let response_body = response.json().await.map_err(ReqwestError::new)?;
                Err(Error::Status401(response_body))
            }
            _ => Err(Error::unknown(response).await),
        }
    }
}

pub mod get_api_v3_account {
    pub use crate::models::get_api_v3_account::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_agg_trades {
    pub use crate::models::get_api_v3_agg_trades::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_all_order_list {
    pub use crate::models::get_api_v3_all_order_list::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_all_orders {
    pub use crate::models::get_api_v3_all_orders::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_avg_price {
    pub use crate::models::get_api_v3_avg_price::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_depth {
    pub use crate::models::get_api_v3_depth::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_exchange_info {
    pub use crate::models::get_api_v3_exchange_info::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_historical_trades {
    pub use crate::models::get_api_v3_historical_trades::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_klines {
    pub use crate::models::get_api_v3_klines::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_my_trades {
    pub use crate::models::get_api_v3_my_trades::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_open_order_list {
    pub use crate::models::get_api_v3_open_order_list::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_open_orders {
    pub use crate::models::get_api_v3_open_orders::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod delete_api_v3_open_orders {
    pub use crate::models::delete_api_v3_open_orders::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_order {
    pub use crate::models::get_api_v3_order::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_api_v3_order {
    pub use crate::models::post_api_v3_order::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod delete_api_v3_order {
    pub use crate::models::delete_api_v3_order::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_api_v3_order_oco {
    pub use crate::models::post_api_v3_order_oco::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_api_v3_order_test {
    pub use crate::models::post_api_v3_order_test::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_order_list {
    pub use crate::models::get_api_v3_order_list::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod delete_api_v3_order_list {
    pub use crate::models::delete_api_v3_order_list::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_ping {
    pub use crate::models::get_api_v3_ping::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_ticker_24hr {
    pub use crate::models::get_api_v3_ticker_24hr::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_ticker_book_ticker {
    pub use crate::models::get_api_v3_ticker_book_ticker::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_ticker_price {
    pub use crate::models::get_api_v3_ticker_price::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_time {
    pub use crate::models::get_api_v3_time::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_api_v3_trades {
    pub use crate::models::get_api_v3_trades::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_api_v3_user_data_stream {
    pub use crate::models::post_api_v3_user_data_stream::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod put_api_v3_user_data_stream {
    pub use crate::models::put_api_v3_user_data_stream::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod delete_api_v3_user_data_stream {
    pub use crate::models::delete_api_v3_user_data_stream::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_account_disable_fast_withdraw_switch {
    pub use crate::models::post_sapi_v1_account_disable_fast_withdraw_switch::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_account_enable_fast_withdraw_switch {
    pub use crate::models::post_sapi_v1_account_enable_fast_withdraw_switch::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_account_snapshot {
    pub use crate::models::get_sapi_v1_account_snapshot::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_asset_asset_dividend {
    pub use crate::models::get_sapi_v1_asset_asset_dividend::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_asset_dust {
    pub use crate::models::post_sapi_v1_asset_dust::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_capital_config_getall {
    pub use crate::models::get_sapi_v1_capital_config_getall::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_capital_deposit_address {
    pub use crate::models::get_sapi_v1_capital_deposit_address::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_capital_deposit_hisrec {
    pub use crate::models::get_sapi_v1_capital_deposit_hisrec::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_capital_withdraw_apply {
    pub use crate::models::post_sapi_v1_capital_withdraw_apply::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_capital_withdraw_history {
    pub use crate::models::get_sapi_v1_capital_withdraw_history::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_account {
    pub use crate::models::get_sapi_v1_margin_account::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_all_assets {
    pub use crate::models::get_sapi_v1_margin_all_assets::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_all_orders {
    pub use crate::models::get_sapi_v1_margin_all_orders::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_all_pairs {
    pub use crate::models::get_sapi_v1_margin_all_pairs::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_asset {
    pub use crate::models::get_sapi_v1_margin_asset::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_force_liquidation_rec {
    pub use crate::models::get_sapi_v1_margin_force_liquidation_rec::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_interest_history {
    pub use crate::models::get_sapi_v1_margin_interest_history::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_loan {
    pub use crate::models::get_sapi_v1_margin_loan::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_margin_loan {
    pub use crate::models::post_sapi_v1_margin_loan::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_max_borrowable {
    pub use crate::models::get_sapi_v1_margin_max_borrowable::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_max_transferable {
    pub use crate::models::get_sapi_v1_margin_max_transferable::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_my_trades {
    pub use crate::models::get_sapi_v1_margin_my_trades::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_open_orders {
    pub use crate::models::get_sapi_v1_margin_open_orders::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_order {
    pub use crate::models::get_sapi_v1_margin_order::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_margin_order {
    pub use crate::models::post_sapi_v1_margin_order::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod delete_sapi_v1_margin_order {
    pub use crate::models::delete_sapi_v1_margin_order::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_pair {
    pub use crate::models::get_sapi_v1_margin_pair::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_price_index {
    pub use crate::models::get_sapi_v1_margin_price_index::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_repay {
    pub use crate::models::get_sapi_v1_margin_repay::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_margin_repay {
    pub use crate::models::post_sapi_v1_margin_repay::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_sapi_v1_margin_transfer {
    pub use crate::models::get_sapi_v1_margin_transfer::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_margin_transfer {
    pub use crate::models::post_sapi_v1_margin_transfer::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod post_sapi_v1_user_data_stream {
    pub use crate::models::post_sapi_v1_user_data_stream::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod put_sapi_v1_user_data_stream {
    pub use crate::models::put_sapi_v1_user_data_stream::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod delete_sapi_v1_user_data_stream {
    pub use crate::models::delete_sapi_v1_user_data_stream::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_wapi_v3_account_status_html {
    pub use crate::models::get_wapi_v3_account_status_html::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_wapi_v3_api_trading_status_html {
    pub use crate::models::get_wapi_v3_api_trading_status_html::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_wapi_v3_asset_detail_html {
    pub use crate::models::get_wapi_v3_asset_detail_html::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_wapi_v3_system_status_html {
    pub use crate::models::get_wapi_v3_system_status_html::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_wapi_v3_trade_fee_html {
    pub use crate::models::get_wapi_v3_trade_fee_html::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}

pub mod get_wapi_v3_user_asset_dribblet_log_html {
    pub use crate::models::get_wapi_v3_user_asset_dribblet_log_html::*;

    #[allow(clippy::large_enum_variant)]
    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Status 200 error: {0:?}
        Status200(Status200),
        /// Status 400 error: {0:?}
        Status400(Status400),
        /// Status 401 error: {0:?}
        Status401(Status401),
        /// Unknown: {headers:?} {text:?}
        Unknown {
            headers: reqwest::header::HeaderMap,
            text: reqwest::Result<String>,
        },
    }

    impl Error {
        pub async fn unknown(response: reqwest::Response) -> Self {
            Self::Unknown {
                headers: response.headers().clone(),
                text: response.text().await,
            }
        }
    }
}
