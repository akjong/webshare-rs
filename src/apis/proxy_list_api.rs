use eyre::Result;

use crate::{
    models::proxy_list::{ProxyList, ProxyListQueryParams, WebshareResult},
    rest::WebShareClient,
};

const PROXY_LIST_API_PATH: &str = "/api/v2/proxy/list";

pub trait ProxyListApi {
    async fn get_proxy_list(
        &self,
        query_params: &ProxyListQueryParams,
    ) -> Result<WebshareResult<ProxyList>>;
}

impl ProxyListApi for WebShareClient {
    async fn get_proxy_list(
        &self,
        query_params: &ProxyListQueryParams,
    ) -> Result<WebshareResult<ProxyList>> {
        let request = self
            .request_builder(reqwest::Method::GET, PROXY_LIST_API_PATH)
            .query(query_params);
        let response = request.send().await?.error_for_status()?;
        let result = response.json::<WebshareResult<ProxyList>>().await?;
        Ok(result)
    }
}
