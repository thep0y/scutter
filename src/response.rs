use crate::request::Request;
use reqwest::Response as RawResponse;

pub struct Response<'a> {
    request: &'a Request<'a>,
    response: RawResponse,
}

impl<'a> Response<'a> {
    pub fn new(request: &Request, response: RawResponse) -> Self {
        Self { request, response }
    }
}
