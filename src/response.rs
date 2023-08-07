use crate::request::Request;
use reqwest::{header::CONTENT_TYPE, Response as RawResponse};

pub struct Response {
    request: Request,
    response: RawResponse,
}

impl Response {
    pub fn new(request: Request, response: RawResponse) -> Self {
        Self { request, response }
    }

    pub fn status_code(self) -> u16 {
        self.response.status().as_u16()
    }

    pub fn content_type(self) -> Option<String> {
        match self.response.headers().get(CONTENT_TYPE) {
            Some(v) => Some(v.to_str().unwrap().to_string()),
            None => None,
        }
    }

    pub fn content_length(self) -> Option<u64> {
        self.response.content_length()
    }
}
