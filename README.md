# Coinchecker

An unofficial Rust Library for the Coincheck REST API.


## Disclaimer

Use at your own risk. (自己責任で使ってください)

- Some APIs not implemented
- Some APIs not well tested


## Usage

If you use the private API, set the access key in the environment variable.
You can use the .env file.

```text:.env
# .env
COINCHECK_ACCESS_KEY=hogehoge
COINCHECK_SECRET_KEY=fugafuga
```

Use like this.

```rust
use anyhow::Result;
use coinchecker::Coincheck;
use coinchecker::types::CoinPair;
use coinchecker::utils::quick_debug;

#[tokio::main]
async fn main() -> Result<()> {
    // Private and Public API
    let mut coincheck = Coincheck::new_with_env_keys();
    quick_debug(coincheck.public.trades(&CoinPair::BtcJpy)).await;
    quick_debug(coincheck.private.account.balance()).await;

    // Public API only
    let mut coincheck = Coincheck::new_without_keys();
    quick_debug(coincheck.public.ticker()).await;

    Ok(())
}
```


## License

MIT License
