use crate::client::SharedClient;
use crate::types::*;
use anyhow::Result;
use reqwest::Method;

/// Private API - Order
///
/// 取引所での注文に関するAPIを利用できます。
///
/// <https://coincheck.com/ja/documents/exchange/api#order>
pub struct Order {
    client: SharedClient,
}

pub mod model {
    use crate::types::*;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, DisplayFromStr};
    use std::collections::HashMap;

    /// 注文結果
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct OrderResultGeneral {
        pub success: bool,
        pub id: IdType,
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub rate: Option<PriceType>,
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub amount: Option<PriceType>,
        pub order_type: String,
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub stop_loss_rate: Option<PriceType>,
        pub pair: String,
        pub created_at: DateTime<Utc>,
    }

    /// 未決済の注文一覧
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OpenOrders {
        pub success: bool,
        pub orders: Vec<OpenOrder>,
    }

    /// 未決済の注文
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OpenOrder {
        pub id: IdType,
        pub order_type: String,
        #[serde_as(as = "DisplayFromStr")]
        pub rate: PriceType,
        pub pair: String,
        #[serde_as(as = "DisplayFromStr")]
        pub pending_amount: PriceType,
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub pending_market_buy_amount: Option<PriceType>,
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub stop_loss_rate: Option<PriceType>,
        pub created_at: DateTime<Utc>,
    }

    /// キャンセル結果
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CancelResult {
        pub success: bool,
        pub id: IdType,
    }

    /// キャンセルステータス
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CancelStatus {
        pub success: bool,
        pub id: IdType,
        pub cancel: bool,
        pub created_at: DateTime<Utc>,
    }

    /// 取引履歴
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderTransactions {
        pub success: bool,
        pub transactions: Vec<OrderTransaction>,
    }

    /// 取引履歴（ページネーション）
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderTransactionsPagination {
        pub success: bool,
        pub pagination: Pagination,
        pub data: Vec<OrderTransaction>,
    }

    /// 取引情報
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderTransaction {
        pub id: IdType,
        pub order_id: IdType,
        pub created_at: DateTime<Utc>,
        #[serde_as(as = "HashMap<DisplayFromStr, DisplayFromStr>")]
        pub funds: HashMap<String, PriceType>,
        pub pair: String,
        #[serde_as(as = "DisplayFromStr")]
        pub rate: PriceType,
        pub fee_currency: Option<String>,
        #[serde_as(as = "DisplayFromStr")]
        pub fee: PriceType,
        pub liquidity: String,
        pub side: String,
    }
}

impl Order {
    pub fn new(client: SharedClient) -> Self {
        Self { client }
    }

    const USE_AUTH: bool = true;

    /// 新規注文 カスタム
    ///
    /// 取引所に新規注文を発行します。リクエストのパラメータを自分で指定します。
    ///
    /// ---
    /// **NOTE**
    ///
    /// 取引所の[注文数量制限](https://faq.coincheck.com/s/article/40218)や[指値注文価格制限](https://faq.coincheck.com/s/article/40217)に引っかかった場合はHTTPエラー `400 Bad Request` になるようです。
    ///
    /// ---
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-new>
    pub async fn new_any(&mut self, params: &Params<'_>) -> Result<model::OrderResultGeneral> {
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::POST,
                "/api/exchange/orders",
                Some(params),
                Self::USE_AUTH,
            )
            .await
    }

    /// 取引所に新規注文を発行します。指値買い (Buy Limit Order) を行います。
    pub async fn new_limit_buy(
        &mut self,
        pair: &CoinPair,
        rate: PriceType,
        amount: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let rate = &rate.to_string();
        let amount = &amount.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::LimitBuy.as_str());
        params.insert("rate", rate);
        params.insert("amount", amount);

        self.new_any(&params).await
    }

    /// 取引所に新規注文を発行します。指値売り (Sell Limit Order) を行います。
    pub async fn new_limit_sell(
        &mut self,
        pair: &CoinPair,
        rate: PriceType,
        amount: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let rate = &rate.to_string();
        let amount = &amount.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::LimitSell.as_str());
        params.insert("rate", rate);
        params.insert("amount", amount);

        self.new_any(&params).await
    }

    /// 取引所に新規注文を発行します。逆指値の指値買い (Buy Stop-Limit Order) を行います。
    pub async fn new_stop_limit_buy(
        &mut self,
        pair: &CoinPair,
        rate: PriceType,
        amount: PriceType,
        stop_loss_rate: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let rate = &rate.to_string();
        let amount = &amount.to_string();
        let stop_loss_rate = &stop_loss_rate.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::LimitBuy.as_str());
        params.insert("rate", rate);
        params.insert("amount", amount);
        params.insert("stop_loss_rate", stop_loss_rate);

        self.new_any(&params).await
    }

    /// 取引所に新規注文を発行します。逆指値の指値売り (Sell Stop-Limit Order) を行います。
    pub async fn new_stop_limit_sell(
        &mut self,
        pair: &CoinPair,
        rate: PriceType,
        amount: PriceType,
        stop_loss_rate: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let rate = &rate.to_string();
        let amount = &amount.to_string();
        let stop_loss_rate = &stop_loss_rate.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::LimitSell.as_str());
        params.insert("rate", rate);
        params.insert("amount", amount);
        params.insert("stop_loss_rate", stop_loss_rate);

        self.new_any(&params).await
    }

    /// 取引所に新規注文を発行します。成行買い (Buy Market Order) を行います。
    pub async fn new_market_buy(
        &mut self,
        pair: &CoinPair,
        amount_jpy: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let amount_jpy = &amount_jpy.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::MarketBuy.as_str());
        params.insert("market_buy_amount", amount_jpy);

        self.new_any(&params).await
    }

    /// 取引所に新規注文を発行します。成行売り (Sell Market Order) を行います。
    pub async fn new_market_sell(
        &mut self,
        pair: &CoinPair,
        amount: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let amount = &amount.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::MarketSell.as_str());
        params.insert("amount", amount);

        self.new_any(&params).await
    }

    /// 取引所に新規注文を発行します。逆指値の成行買い (Buy Stop-Market Order) を行います。
    pub async fn new_stop_market_buy(
        &mut self,
        pair: &CoinPair,
        amount_jpy: PriceType,
        stop_loss_rate: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let amount_jpy = &amount_jpy.to_string();
        let stop_loss_rate = &stop_loss_rate.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::MarketBuy.as_str());
        params.insert("market_buy_amount", amount_jpy);
        params.insert("stop_loss_rate", stop_loss_rate);

        self.new_any(&params).await
    }

    /// 取引所に新規注文を発行します。逆指値の成行売り (Sell Stop-Market Order) を行います。
    pub async fn new_stop_market_sell(
        &mut self,
        pair: &CoinPair,
        amount: PriceType,
        stop_loss_rate: PriceType,
    ) -> Result<model::OrderResultGeneral> {
        let mut params = Params::new();
        let amount = &amount.to_string();
        let stop_loss_rate = &stop_loss_rate.to_string();
        params.insert("pair", pair.as_str());
        params.insert("order_type", OrderType::MarketSell.as_str());
        params.insert("amount", amount);
        params.insert("stop_loss_rate", stop_loss_rate);

        self.new_any(&params).await
    }

    /// 未決済の注文一覧
    ///
    /// アカウントの未決済の注文を一覧で表示します。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-opens>
    pub async fn opens(&mut self) -> Result<model::OpenOrders> {
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/exchange/orders/opens",
                None,
                Self::USE_AUTH,
            )
            .await
    }

    /// 注文のキャンセル
    ///
    /// 新規注文または未決済の注文一覧のIDを指定してキャンセルすることができます。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-cancel>
    pub async fn cancel(&mut self, id: IdType) -> Result<model::CancelResult> {
        let url = format!("/api/exchange/orders/{}", id);
        self.client
            .borrow_mut()
            .request_and_get_json(Method::DELETE, &url, None, Self::USE_AUTH)
            .await
    }

    /// 注文のキャンセルステータス
    ///
    /// オーダーのキャンセル処理状況を参照出来ます。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#cancel-status>
    pub async fn cancel_status(&mut self, id: IdType) -> Result<model::CancelStatus> {
        let mut params = Params::new();
        let id: &str = &id.to_string();
        params.insert("id", id);
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/exchange/orders/cancel_status",
                Some(&params),
                Self::USE_AUTH,
            )
            .await
    }

    /// 取引履歴
    ///
    /// 自分の最近の取引履歴を参照できます。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-transactions>
    pub async fn transactions(&mut self) -> Result<model::OrderTransactions> {
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/exchange/orders/transactions",
                None,
                Self::USE_AUTH,
            )
            .await
    }

    /// 取引履歴（ページネーション）
    ///
    /// 自分の最近の取引履歴を参照できます。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#order-transactions-pagination>
    pub async fn transactions_pagination(
        &mut self,
        pagination: Pagination,
    ) -> Result<model::OrderTransactionsPagination> {
        let mut params = Params::new();
        let limit: &str = &pagination.limit.to_string();
        let order: &str = &pagination.order.to_string();
        params.insert("limit", limit);
        params.insert("order", order);

        let tmp_str; // to create a longer lived value
        if let Some(r) = pagination.starting_after {
            tmp_str = r.to_string();
            params.insert("starting_after", &tmp_str);
        };

        let tmp_str; // to create a longer lived value
        if let Some(r) = pagination.ending_before {
            tmp_str = r.to_string();
            params.insert("ending_before", &tmp_str);
        };

        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/exchange/orders/transactions_pagination",
                Some(&params),
                Self::USE_AUTH,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use crate::Coincheck;

    // Warn: THE BUY/SELL/CANCEL FUNCTIONS ARE NOT TESTED HERE!
    #[tokio::test]
    #[serial_test::serial]
    async fn private_order_api() {
        let mut coincheck = Coincheck::new_with_env_keys();
        let api = &mut coincheck.private.order;

        assert!(api.opens().await.is_ok());
        assert!(api.transactions().await.is_ok());
        assert!(api
            .transactions_pagination(Pagination {
                limit: 3,
                order: SortOrder::Asc,
                starting_after: None,
                ending_before: None,
            })
            .await
            .is_ok());
    }
}
