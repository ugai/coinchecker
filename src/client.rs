use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Instant, SystemTime};

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Method, Url};

use serde::de::DeserializeOwned;

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

use anyhow::{anyhow, Result};

use crate::types::*;

const API_BASE: &str = "https://coincheck.com";

struct Header {}
impl Header {
    const NONCE: &'static str = "ACCESS-NONCE";
    const SIGNATURE: &'static str = "ACCESS-SIGNATURE";
    const KEY: &'static str = "ACCESS-KEY";
}

pub type SharedClient = Rc<RefCell<Client>>;

pub struct Client {
    access_key: Option<String>,
    secret_key: Option<String>,
    client: reqwest::Client,
    pub last_request_time: Instant,
}

/// A simple wrapper of the HTTP client.
impl Client {
    /// Create a new instance.
    pub fn shared_new(access_key: Option<String>, secret_key: Option<String>) -> SharedClient {
        Rc::new(RefCell::new(Client {
            access_key,
            secret_key,
            client: reqwest::Client::builder().https_only(true).build().unwrap(),
            last_request_time: Instant::now(),
        }))
    }

    /// Create authentication HTTP header for the Coincheck REST API .
    fn set_auth_headers(&mut self, headers: &mut HeaderMap, url: &Url) -> Result<()> {
        let nonce = Client::get_nonce()?;
        headers.insert(Header::NONCE, HeaderValue::from_str(&nonce).unwrap());

        let message = nonce.to_owned() + url.as_str();
        let signature = Client::get_signature(&self.secret_key.as_ref().unwrap(), &message)?;
        headers.insert(Header::SIGNATURE, signature.parse().unwrap());
        headers.insert(
            Header::KEY,
            self.access_key.as_ref().unwrap().parse().unwrap(),
        );

        Ok(())
    }

    /// Get nonce for authentication header creation.
    fn get_nonce() -> Result<String> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(dur) => Ok(dur.as_micros().to_string()),
            Err(_) => Err(anyhow!("SystemTime before UNIX EPOCH!")),
        }
    }

    /// Get signature for authentication header creation.
    fn get_signature(secret_key: &str, message: &str) -> Result<String> {
        match HmacSha256::new_from_slice(secret_key.as_bytes()) {
            Ok(mut mac) => {
                mac.update(message.as_bytes());
                Ok(hex::encode(mac.finalize().into_bytes()))
            }
            Err(_) => Err(anyhow!("invalid key length for MAC initialization")),
        }
    }

    /// Send a request to the API and get a JSON result.
    pub async fn request_and_get_json<T: DeserializeOwned>(
        &mut self,
        method: Method,
        path: &str,
        params: Option<&Params<'_>>,
        use_auth: bool,
    ) -> Result<T> {
        let res = self.request(method, &path, params, use_auth).await?;
        let data = res.json().await?;
        Ok(data)
    }

    /// Send a request to the API and get a decoded text.
    pub async fn request_and_get_text(
        &mut self,
        method: Method,
        path: &str,
        params: Option<&Params<'_>>,
        use_auth: bool,
    ) -> Result<String> {
        let res = self.request(method, &path, params, use_auth).await?;
        let data = res.text().await?;
        Ok(data)
    }

    /// Send a request to the API.
    pub async fn request(
        &mut self,
        method: Method,
        path: &str,
        params: Option<&Params<'_>>,
        use_auth: bool,
    ) -> Result<reqwest::Response> {
        self.last_request_time = Instant::now();

        let url = API_BASE.to_owned() + path;

        let url = if let Some(params) = params {
            Url::parse_with_params(&url, params).unwrap()
        } else {
            Url::parse(&url).unwrap()
        };
        let mut headers = HeaderMap::new();
        if use_auth {
            self.set_auth_headers(&mut headers, &url).unwrap()
        }

        const CONTENT_TYPE_VALUE_JSON: &str = "application/json";
        if method == Method::POST || method == Method::DELETE {
            headers.insert(CONTENT_TYPE, CONTENT_TYPE_VALUE_JSON.parse().unwrap());
        }

        let res = match method {
            Method::GET => self.client.get(url).headers(headers).send().await,
            Method::POST => self.client.post(url).headers(headers).send().await,
            Method::DELETE => self.client.delete(url).headers(headers).send().await,
            _ => {
                return Err(anyhow!("unsupported http method type"));
            }
        }?;

        res.error_for_status_ref()?;

        Ok(res)
    }
}
