use crate::client::SharedClient;
use crate::types::*;
use anyhow::Result;
use reqwest::Method;

/// Private API - Account
///
/// 自分のアカウントの残高や、各種情報を取得することができます。
///
/// <https://coincheck.com/ja/documents/exchange/api#account>
pub struct Account {
    client: SharedClient,
}

pub mod model {
    use crate::types::*;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, DisplayFromStr};
    use std::collections::HashMap;

    /// 残高
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Balance {
        pub success: bool,
        #[serde_as(as = "DisplayFromStr")]
        pub jpy: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub btc: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub jpy_reserved: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub btc_reserved: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub jpy_lend_in_use: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub btc_lend_in_use: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub jpy_lent: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub btc_lent: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub jpy_debt: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub btc_debt: PriceType,
    }

    /// 送金履歴
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SendHistory {
        pub success: bool,
        pub sends: Vec<SendRecord>,
    }

    /// 送金履歴のレコード
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SendRecord {
        pub id: IdType,
        #[serde_as(as = "DisplayFromStr")]
        pub amount: PriceType,
        pub currency: String,
        #[serde_as(as = "DisplayFromStr")]
        pub fee: PriceType,
        pub address: String,
        pub created_at: DateTime<Utc>,
    }

    /// 受け取り履歴
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DepositHistory {
        pub success: bool,
        pub deposits: Vec<DepositRecord>,
    }

    /// 受け取り履歴のレコード
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DepositRecord {
        pub id: IdType,
        #[serde_as(as = "DisplayFromStr")]
        pub amount: PriceType,
        pub currency: String,
        pub address: String,
        pub status: String,
        pub confirmed_at: String,
        pub created_at: DateTime<Utc>,
    }

    /// アカウント情報
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Account {
        pub success: bool,
        pub id: IdType,
        pub email: String,
        pub identity_status: String,
        pub bitcoin_address: String,
        #[serde_as(as = "DisplayFromStr")]
        pub taker_fee: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub maker_fee: PriceType,
        pub exchange_fees: HashMap<String, Fee>,
    }

    /// 手数料
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Fee {
        #[serde_as(as = "DisplayFromStr")]
        pub taker_fee: PriceType,
        #[serde_as(as = "DisplayFromStr")]
        pub maker_fee: PriceType,
    }
}

impl Account {
    pub fn new(client: SharedClient) -> Self {
        Self { client }
    }

    const USE_AUTH: bool = true;

    /// 残高
    ///
    /// アカウントの残高を確認できます。
    /// jpy, btc には未決済の注文に利用している jpy_reserved, btc_reserved は含まれていません。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#account-balance>
    pub async fn balance(&mut self) -> Result<model::Balance> {
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, "/api/accounts/balance", None, Self::USE_AUTH)
            .await
    }

    // TODO: implement ビットコインの送金 POST /api/send_money
    // https://coincheck.com/ja/documents/exchange/api#account-sendmoney

    /// 送金履歴
    ///
    /// ビットコインの送金履歴です。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#account-sends>
    pub async fn sends(&mut self) -> Result<model::SendHistory> {
        let mut params = Params::new();
        params.insert("currency", Currency::Btc.as_str());
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/send_money",
                Some(&params),
                Self::USE_AUTH,
            )
            .await
    }

    /// 受け取り履歴
    ///
    /// ビットコインの受け取り履歴です。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#account-deposits>
    pub async fn deposits(&mut self) -> Result<model::DepositHistory> {
        let mut params = Params::new();
        params.insert("currency", Currency::Btc.as_str());
        self.client
            .borrow_mut()
            .request_and_get_json(
                Method::GET,
                "/api/deposit_money",
                Some(&params),
                Self::USE_AUTH,
            )
            .await
    }

    /// アカウント情報
    ///
    /// アカウントの情報を表示します。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#account-info>
    pub async fn info(&mut self) -> Result<model::Account> {
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, "/api/accounts", None, Self::USE_AUTH)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Coincheck;

    #[tokio::test]
    #[serial_test::serial]
    async fn private_account_api() {
        let mut coincheck = Coincheck::new_with_env_keys();
        let api = &mut coincheck.private.account;

        assert!(api.balance().await.is_ok());
        assert!(api.sends().await.is_ok());
        assert!(api.deposits().await.is_ok());
        assert!(api.info().await.is_ok());
    }
}
