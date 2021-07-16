use crate::client::SharedClient;
use anyhow::Result;
use reqwest::Method;

/// Private API - Withdraws JPY
///
/// 日本円を銀行振込で出金できます。
///
/// <https://coincheck.com/ja/documents/exchange/api#withdraws-jpy>
pub struct WithdrawsJpy {
    client: SharedClient,
}

pub mod model {
    use crate::types::*;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, DisplayFromStr};

    /// 銀行口座一覧
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BankAccounts {
        pub success: bool,
        pub data: Vec<BankAccount>,
    }

    /// 銀行口座情報
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BankAccount {
        pub id: IdType,
        pub bank_name: String,
        pub branch_name: String,
        pub bank_account_type: String,
        pub number: String,
        pub name: String,
    }

    /// 出金履歴
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Withdraws {
        pub success: bool,
        pub pagination: Pagination,
        pub data: Vec<Withdraw>,
    }

    /// 出金情報
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Withdraw {
        pub id: IdType,
        pub status: String,
        #[serde_as(as = "DisplayFromStr")]
        pub amount: PriceType,
        pub currency: String,
        pub created_at: DateTime<Utc>,
        pub bank_account_id: IdType,
        #[serde_as(as = "DisplayFromStr")]
        pub fee: PriceType,
        pub is_fast: bool,
    }
}

impl WithdrawsJpy {
    pub fn new(client: SharedClient) -> Self {
        Self { client }
    }

    const USE_AUTH: bool = true;

    /// 銀行口座一覧
    ///
    /// お客様の出金用に登録された銀行口座の一覧を返します。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#bank-accounts>
    pub async fn bank_accounts(&mut self) -> Result<model::BankAccounts> {
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, "/api/bank_accounts", None, Self::USE_AUTH)
            .await
    }

    // TODO: implement 銀行口座の登録 POST /api/bank_accounts
    // https://coincheck.com/ja/documents/exchange/api#bank-accounts-create
    // TODO: implement 銀行口座の削除 DELETE /api/bank_accounts/[id]
    // https://coincheck.com/ja/documents/exchange/api#bank-accounts-destroy

    /// 出金履歴
    ///
    /// 日本円出金の申請の履歴を表示します。
    ///
    /// <https://coincheck.com/ja/documents/exchange/api#withdraws>
    pub async fn withdraws(&mut self) -> Result<model::Withdraws> {
        self.client
            .borrow_mut()
            .request_and_get_json(Method::GET, "/api/withdraws", None, Self::USE_AUTH)
            .await
    }

    // TODO: implement 出金申請の作成 POST /api/withdraws
    // https://coincheck.com/ja/documents/exchange/api#withdraws-create
    // TODO: implement 出金申請のキャンセル DELETE /api/withdraws/[id]
    // https://coincheck.com/ja/documents/exchange/api#withdraws-destroy
}

#[cfg(test)]
mod tests {
    use crate::Coincheck;

    #[tokio::test]
    #[serial_test::serial]
    async fn private_withdraw_jpy_api() {
        let mut coincheck = Coincheck::new_with_env_keys();
        let api = &mut coincheck.private.withdraws_jpy;

        assert!(api.bank_accounts().await.is_ok());
        assert!(api.withdraws().await.is_ok());
    }
}
