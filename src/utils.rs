use anyhow::Result;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::Request;

pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    pub fn from_request(req: &Request) -> Result<Self> {
        let url = Url::parse(&req.uri().to_string())?;
        let params = url.query_pairs().into_owned().collect();

        Ok(Self { params })
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}
