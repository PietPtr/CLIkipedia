use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Response,
};
use std::{error::Error, future::Future};

pub struct Wikipedia {
    client: reqwest::Client,
}

impl Wikipedia {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str("clikipedia/1.0 (pieterstaal@outlook.com)").unwrap(),
        );
        headers
    }

    fn get(&self, endpoint: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
        self.client
            .get(format!(
                "https://en.wikipedia.org/api/rest_v1/page/{endpoint}"
            ))
            .headers(Wikipedia::headers())
            .send()
    }

    pub async fn random_page(&self) -> Result<String, Box<dyn Error>> {
        let response = self.get("random/html".to_string()).await?;
        let body = response.text().await?;

        Ok(body)
    }
}
