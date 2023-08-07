use bytes::Bytes;
use reqwest::Method;
use tokio::time::Duration;
use url::Url;

pub struct Request {
    id: u32,
    method: Method,
    url: Url,
    body: Option<Bytes>,
    timeout: Duration,
}

impl Request {
    pub(crate) fn new(
        method: Method,
        url: Url,
        body: Option<Bytes>,
        id: u32,
        timeout: u32,
    ) -> Self {
        Self {
            method,
            url,
            body,
            id,
            timeout: Duration::from_millis(timeout.into()),
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn timeout(self) -> Duration {
        self.timeout
    }
}
