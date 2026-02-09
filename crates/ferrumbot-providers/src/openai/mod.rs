use std::collections::BTreeMap;

use reqwest::Client;

pub struct OpenAiCompatibleProvider {
    pub(super) client: Client,
    pub(super) api_key: String,
    pub(super) api_base: String,
    pub(super) default_model: String,
    pub(super) extra_headers: BTreeMap<String, String>,
}

mod build;
mod call;
mod model;
mod parse;
