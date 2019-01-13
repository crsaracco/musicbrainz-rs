use std::time::Duration;
use ratelimit;
use reqwest::{
    self,
    header::*,
    ClientBuilder,
};

pub struct ReqwestClient {
    reqwest_client: reqwest::Client,
    rate_limiter: ratelimit::Limiter,
}

impl ReqwestClient {
    pub fn new(user_agent: &str) -> Result<Self, InvalidHeaderValue> {
        // Set default client headers
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);

        // Build a client
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        // Build a ratelimiter
        let rate_limiter = ratelimit::Builder::new()
            .capacity(1)
            .quantum(1)
            .interval(Duration::new(1, 0))
            .build();

        // Return new RequestClient struct
        Ok(Self {
            reqwest_client: client,
            rate_limiter,
        })
    }

    pub fn get(&mut self, url: &str) -> Result<String, reqwest::Error> {
        // Wait for a ratelimit token
        self.rate_limiter.wait();

        // Build a request using the client
        let request = self.reqwest_client.get(url).build()?;

        // Fetch the URL and get a response
        let mut response = self.reqwest_client.execute(request)?;

        // Get the text (body) out of the response
        let text = response.text()?;

        Ok(text)
    }
}