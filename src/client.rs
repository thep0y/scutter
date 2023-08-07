use crate::{error::Result, request::Request, response::Response};
use bytes::Bytes;
use reqwest::{
    header::{CONTENT_TYPE, USER_AGENT},
    Client as RawClient, IntoUrl, Method, Request as RawRequest,
};
use std::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

pub struct Client {
    user_agent: String,
    client: RawClient,
    request_counter: AtomicU32,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            user_agent: "Scutter".into(),
            client: RawClient::new(),
            request_counter: AtomicU32::new(1),
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_user_agent<S: AsRef<str>>(&mut self, user_agent: S) {
        self.user_agent = user_agent.as_ref().into();
    }

    fn increase_request_counter(&self) -> u32 {
        self.request_counter.fetch_add(1, Ordering::SeqCst)
    }

    async fn request<U: IntoUrl>(
        &self,
        method: Method,
        u: U,
        body: Option<Bytes>,
        timeout: u32,
    ) -> Result<Response> {
        let url = u.into_url()?;

        let id = self.increase_request_counter();
        let req = Request::new(method.clone(), url.clone(), None, id, timeout);
        debug!("sending a request: id={} method={}", id, method);

        let mut rb = self
            .client
            .request(method, url.clone())
            .header(USER_AGENT, &self.user_agent)
            .timeout(Duration::from_millis(timeout.into()));

        if let Some(body) = body {
            rb = rb.body(body);
        }

        let response = rb.send().await?;

        debug!(
            "got a response: status={} content-type={:?} content-length={}",
            response.status(),
            response.headers().get(CONTENT_TYPE),
            response.content_length().unwrap_or(0)
        );

        Ok(Response::new(req, response))
    }

    pub async fn get<U: IntoUrl>(&mut self, url: U) -> Result<Response> {
        self.request(Method::GET, url, None, 0).await
    }
}
