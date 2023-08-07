use crate::request::Request;
use reqwest::Response as RawResponse;

pub struct Response {
    request: Request,
    response: RawResponse,
}

impl Response {
    pub fn new(request: Request, response: RawResponse) -> Self {
        Self { request, response }
    }
}
