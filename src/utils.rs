use anyhow::Result;

/// Run async function and output the result. For quick API checking.
///
/// # Examples
///
/// ```rust
/// # use coinchecker::Coincheck;
/// # use coinchecker::utils::quick_debug;
/// #
/// #[tokio::main]
/// async fn main() {
///     let mut coincheck = Coincheck::new_with_env_keys();
///     quick_debug(coincheck.public.ticker()).await;
///     // output:
///     //   Ticker { last: 4043996.0, bid: 4043000.0, ...
/// }
/// ```
pub async fn quick_debug<T, F>(task: F) -> ()
where
    T: std::fmt::Debug,
    F: std::future::Future<Output = Result<T>>,
{
    match task.await {
        Ok(data) => println!("{:?}", data),
        Err(err) => println!("error: {}", err),
    };
}
