//! # Coinchecker
//!
//! An unofficial Rust Library for the Coincheck REST API.
//!
//!
//! ## Disclaimer
//!
//! Use at your own risk. (自己責任で使ってください)
//!
//! - Some APIs not implemented
//! - Some APIs not well tested
//!
//!
//! ## APIs
//!
//! - [Public]
//! - [Private]
//!     - [Account]
//!     - [Order]
//!     - [WithdrawsJpy]
//!
//!
//! ## Usage
//!
//! If you use the private API, set the access key in the environment variable.
//! You can use the .env file.
//!
//! ```text:.env
//! # .env file
//! COINCHECK_ACCESS_KEY=hogehoge
//! COINCHECK_SECRET_KEY=fugafuga
//! ```
//!
//! Use like this.
//!
//! ```rust
//! use anyhow::Result;
//! use coinchecker::Coincheck;
//! use coinchecker::types::CoinPair;
//! use coinchecker::utils::quick_debug;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Private and Public API
//!     let mut coincheck = Coincheck::new_with_env_keys();
//!     quick_debug(coincheck.public.trades(&CoinPair::BtcJpy)).await;
//!     quick_debug(coincheck.private.account.balance()).await;
//!
//!     // Public API only
//!     let mut coincheck = Coincheck::new_without_keys();
//!     quick_debug(coincheck.public.ticker()).await;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod private;
pub mod public;
pub mod types;
pub mod utils;

use client::SharedClient;

use crate::client::Client;
use crate::private::account::Account;
use crate::private::order::Order;
use crate::private::withdraws_jpy::WithdrawsJpy;
use crate::private::Private;
use crate::public::Public;
use std::time::Instant;

/// A client for the Coincheck REST API.
pub struct Coincheck {
    pub public: Public,
    pub private: Private,
    client: SharedClient,
}

/// A client for the Coincheck REST API. Without API keys. Public API only.
pub struct CoincheckNoAuth {
    pub public: Public,
    client: SharedClient,
}

impl Coincheck {
    pub const ENV_ACCESS_KEY: &'static str = "COINCHECK_ACCESS_KEY";
    pub const ENV_SECRET_KEY: &'static str = "COINCHECK_SECRET_KEY";

    /// Create a new instance.
    pub fn new_with_keys(access_key: &str, secret_key: &str) -> Coincheck {
        let client = Client::shared_new(access_key.to_owned().into(), secret_key.to_owned().into());
        let public = Public::new(client.clone());

        let private = Private {
            order: Order::new(client.clone()),
            account: Account::new(client.clone()),
            withdraws_jpy: WithdrawsJpy::new(client.clone()),
        };

        Coincheck {
            public,
            private,
            client,
        }
    }

    /// Create a new instance. Use the authentication key from the environment variables (needs `COINCHECK_ACCESS_KEY` and `COINCHECK_SECRET_KEY`).
    pub fn new_with_env_keys() -> Coincheck {
        dotenv::dotenv().ok();

        let access_key = std::env::var(Self::ENV_ACCESS_KEY)
            .unwrap_or_else(|_| panic!("{} must be set", Self::ENV_ACCESS_KEY));
        let secret_key = std::env::var(Self::ENV_SECRET_KEY)
            .unwrap_or_else(|_| panic!("{} must be set", Self::ENV_SECRET_KEY));

        Coincheck::new_with_keys(&access_key, &secret_key)
    }

    /// Create a new instance without authentication keys. Only public APIs can be used.
    pub fn new_without_keys() -> CoincheckNoAuth {
        let client = Client::shared_new(None, None);
        let public = Public::new(client.clone());

        CoincheckNoAuth { public, client }
    }
}

trait GetLastRequestTime {
    /// Get the last requset time.
    fn last_request_time(&self) -> Instant;
}

impl GetLastRequestTime for Coincheck {
    fn last_request_time(&self) -> Instant {
        self.client.borrow().last_request_time
    }
}

impl GetLastRequestTime for CoincheckNoAuth {
    fn last_request_time(&self) -> Instant {
        self.client.borrow().last_request_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_coincheck_instance() {
        let _ = Coincheck::new_with_keys("hoge", "fuga");
        let _ = Coincheck::new_with_env_keys();
        let _ = Coincheck::new_without_keys();
    }
}
