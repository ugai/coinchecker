use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

pub type Params<'a> = HashMap<&'a str, &'a str>;

/// ID value type (default: [u64])
pub type IdType = u64;

/// Price value type (default: [f64])
#[cfg(not(feature = "price_type_f32"))]
pub type PriceType = f64;
#[cfg(feature = "price_type_f32")]
pub type PriceType = f32;

/// 通貨
pub enum Currency {
    Jpy,
    Btc,
}

impl Currency {
    pub const fn as_str(&self) -> &str {
        match self {
            Currency::Jpy => "JPY",
            Currency::Btc => "BTC",
        }
    }
}

/// 取引ペア
pub enum CoinPair {
    BtcJpy,
    EtcJpy,
    FctJpy,
    MonaJpy,
    PltJpy,
}

impl CoinPair {
    pub const fn as_str(&self) -> &str {
        match self {
            CoinPair::BtcJpy => "btc_jpy",
            CoinPair::EtcJpy => "etc_jpy",
            CoinPair::FctJpy => "fct_jpy",
            CoinPair::MonaJpy => "mona_jpy",
            CoinPair::PltJpy => "plt_jpy",
        }
    }
}

/// 売り買い
pub enum BaseOrderType {
    Buy,
    Sell,
}

impl BaseOrderType {
    pub fn as_str(&self) -> &str {
        match self {
            BaseOrderType::Buy => "buy",
            BaseOrderType::Sell => "sell",
        }
    }
}

/// 注文方法
pub enum OrderType {
    Limit(BaseOrderType),
    MarketBuy,
    MarketSell,
}
impl OrderType {
    #[allow(non_upper_case_globals)]
    pub const LimitBuy: OrderType = OrderType::Limit(BaseOrderType::Buy);
    #[allow(non_upper_case_globals)]
    pub const LimitSell: OrderType = OrderType::Limit(BaseOrderType::Sell);

    pub fn as_str(&self) -> &str {
        match self {
            OrderType::Limit(base) => base.as_str(),
            OrderType::MarketBuy => "market_buy",
            OrderType::MarketSell => "market_sell",
        }
    }
}

/// 並び順
#[derive(Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}
impl SortOrder {
    pub fn as_str(&self) -> &str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for SortOrder {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == SortOrder::Asc.as_str() {
            return Ok(SortOrder::Asc);
        } else if s == SortOrder::Desc.as_str() {
            return Ok(SortOrder::Desc);
        }

        Err("undefined SortOrder type")
    }
}

/// ページネーション
///
/// <https://coincheck.com/ja/documents/exchange/api#pagination>
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: i32,
    #[serde_as(as = "DisplayFromStr")]
    pub order: SortOrder,
    pub starting_after: Option<IdType>,
    pub ending_before: Option<IdType>,
}
