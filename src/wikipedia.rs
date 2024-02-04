use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Response,
};
use std::{env, error::Error, future::Future};

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
        let email = match env::var("USER_EMAIL") {
            Ok(ref s) if s.is_empty() => {
                panic!("No e-mail env var set. Set USER_EMAIL to your e-mail address.")
            }
            Ok(email) => email,
            Err(_) => panic!("Cannot read env var USER_EMAIL."),
        };
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(format!("clikipedia/1.0 ({})", email).as_str()).unwrap(),
        );
        headers
    }

    pub fn get(&self, endpoint: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
        self.client
            .get(format!(
                "https://en.wikipedia.org/api/rest_v1/page/{endpoint}"
            ))
            .headers(Wikipedia::headers())
            .send()
    }

    pub async fn get_page(&self, page: &str) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .get(format!(
                "https://en.wikipedia.org/api/rest_v1/page/html/{}",
                page
            ))
            .headers(Wikipedia::headers())
            .send()
            .await?;

        let html = response.text().await?;
        Ok(html)
    }

    pub async fn random_page(&self) -> Result<String, Box<dyn Error>> {
        let response = self.get("random/html".to_string()).await?;
        let body = response.text().await?;

        Ok(body)
    }
}
