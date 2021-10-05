use crate::client::SharedClient;
use crate::types::*;
use anyhow::Result;
use reqwest::Method;

/// Public API
///
/// 取引所の注文状況や公開されている取引の履歴、板情報を参照することができます。
///
/// <https://coincheck.com/ja/documents/exchange/api#public>
pub struct Public {
    client: SharedClient,
}

mod model {
    use crate::types::*;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, DisplayFromStr, TimestampMilliSeconds};

    /// ティッカー
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ticker {
        pub last: PriceType,
        pub bid: PriceType,
        pub ask: PriceType,
        pub high: PriceType,
        pub low: PriceType,
        pub volume: PriceType,
        #[serde_as(as = "TimestampMilliSeconds")]
        pub timestamp: DateTime<Utc>,
    }

    /// 全取引履歴
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Trades {
        pub success: bool,
        pub pagination: Pagination,
        pub data: Vec<Trade>,
    }

    /// 取引情報
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Trade {
        pub id: IdType,
        pub amount: String,
        pub rate: String,
        pub pair: String,
        pub order_type: String,
        pub created_at: DateTime<Utc>,
    }

    /// 板情報
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderBooks {
        pub asks: Vec<OrderBook>,
        pub bids: Vec<OrderBook>,
    }

    /// 注文情報
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderBook {
        #[serde_as(as = "DisplayFromStr")]
        pub rate: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub amount: PriceType,
    }

    /// レート算出結果
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CalculatedRate {
        pub success: bool,
        #[serde_as(as = "DisplayFromStr")]
        pub rate: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub price: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub amount: PriceType,
    }

    /// 販売所レート情報
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExchangeRate {
        #[serde_as(as = "DisplayFromStr")]
        pub rate: PriceType,
    }
}

impl Public {
    pub fn new(client: SharedClient) -> Self {
        Self { client }
    }

    const USE_AUTH: bool = false;

    /// ティッカー
    ///
    /// 各種最新情報を簡易に取得することができます。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#ticker>
    pub async fn ticker(&mut self) -> Result<model::Ticker> {
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, "/api/ticker", None, Self::USE_AUTH)
            .await
    }

    /// 全取引履歴
    ///
    /// 最新の取引履歴を取得できます。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#public-trades>
    pub async fn trades(&mut self, pair: &CoinPair) -> Result<model::Trades> {
        let mut params = Params::new();
        params.insert("pair", pair.as_str());
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, "/api/trades", Some(&params), Self::USE_AUTH)
            .await
    }

    /// 板情報
    ///
    /// 板情報を取得できます。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-book>
    pub async fn order_book(&mut self) -> Result<model::OrderBooks> {
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, "/api/order_books", None, Self::USE_AUTH)
            .await
    }

    /// レート取得
    ///
    /// 取引所の注文を元にレートを算出します。注文量を使用します。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-rate>
    pub async fn order_rate_from_amount(
        &mut self,
        order_type: &BaseOrderType,
        pair: &CoinPair,
        amount: PriceType,
    ) -> Result<model::CalculatedRate> {
        let mut params = Params::new();
        let amount = amount.to_string();
        params.insert("order_type", order_type.as_str());
        params.insert("pair", pair.as_str());
        params.insert("amount", &amount);
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/exchange/orders/rate",
                Some(&params),
                Self::USE_AUTH,
            )
            .await
    }

    /// レート取得
    ///
    /// 取引所の注文を元にレートを算出します。注文金額を使用します。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-rate>
    pub async fn order_rate_from_price(
        &mut self,
        order_type: &BaseOrderType,
        pair: &CoinPair,
        price: PriceType,
    ) -> Result<model::CalculatedRate> {
        let mut params = Params::new();
        let price = price.to_string();
        params.insert("order_type", order_type.as_str());
        params.insert("pair", pair.as_str());
        params.insert("price", &price);
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/exchange/orders/rate",
                Some(&params),
                Self::USE_AUTH,
            )
            .await
    }

    /// 販売レート取得
    ///
    /// 販売所のレートを取得します。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#buy-rate>
    pub async fn marketplace_buy_rate<'a>(
        &mut self,
        pair: &CoinPair,
    ) -> Result<model::ExchangeRate> {
        let url = format!("/api/rate/{}", pair.as_str());
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, &url, None, Self::USE_AUTH)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use crate::Coincheck;

    #[tokio::test]
    #[serial_test::serial]
    async fn public_api() {
        let mut coincheck = Coincheck::new_without_keys();
        let api = &mut coincheck.public;

        let pair = &CoinPair::BtcJpy;
        let otype = &BaseOrderType::Buy;

        assert!(api.ticker().await.is_ok());
        assert!(api.trades(pair).await.is_ok());
        assert!(api.order_book().await.is_ok());
        assert!(api
            .order_rate_from_amount(otype, pair, 0.1 as PriceType)
            .await
            .is_ok());
        assert!(api
            .order_rate_from_price(otype, pair, 35000 as PriceType)
            .await
            .is_ok());
        assert!(api.marketplace_buy_rate(pair).await.is_ok());
    }
}
