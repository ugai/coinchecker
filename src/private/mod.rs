pub mod account;
pub mod order;
pub mod withdraws_jpy;

use crate::private::account::Account;
use crate::private::order::Order;
use crate::private::withdraws_jpy::WithdrawsJpy;

/// Private API
///
/// 取引所での新規注文やそのキャンセル、自分の残高などを確認することができます。
///
/// <https://coincheck.com/ja/documents/exchange/api#private>
pub struct Private {
    pub order: Order,
    pub account: Account,
    pub withdraws_jpy: WithdrawsJpy,
}
