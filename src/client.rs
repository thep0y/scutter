use reqwest::{header::USER_AGENT, Client as rawClient, IntoUrl};

pub struct Client {
    user_agent: String,
    client: rawClient,
    request_counter: u32,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            user_agent: "Scutter".into(),
            client: rawClient::new(),
            request_counter: 0,
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

    pub async fn get<U: IntoUrl>(&mut self, url: U) {
        let respone = self
            .client
            .get(url)
            .header(USER_AGENT, &self.user_agent)
            .send()
            .await
            .unwrap();

        self.request_counter += 1;

        debug!("got a response: status-code={}", respone.status());
    }
}
