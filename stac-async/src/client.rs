use crate::Error;
use reqwest::IntoUrl;
use stac::{Href, Value};

/// A thin wrapper around [reqwest::Client].
#[derive(Clone, Debug)]
pub struct Client(pub reqwest::Client);

impl Client {
    /// Creates a new client.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = stac_async::Client::new();
    /// ```
    ///
    /// ## Custom client
    ///
    /// You can construct the client directly using a pre-built
    /// [reqwest::Client], e.g. to do authorization:
    ///
    /// ```
    /// use reqwest::header;
    /// let mut headers = header::HeaderMap::new();
    /// let mut auth_value = header::HeaderValue::from_static("secret");
    /// auth_value.set_sensitive(true);
    /// headers.insert(header::AUTHORIZATION, auth_value);
    /// let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
    /// let client = stac_async::Client(client);
    /// ```
    pub fn new() -> Client {
        Client(reqwest::Client::new())
    }

    /// Gets a STAC value from a url.
    ///
    /// Also sets that [Values](Value) href.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = stac_async::Client::new();
    /// let href = "https://raw.githubusercontent.com/radiantearth/stac-spec/v1.0.0/examples/simple-item.json";
    /// # tokio_test::block_on(async {
    /// let value = client.get(href).await.unwrap();
    /// # })
    /// ```
    pub async fn get(&self, url: impl IntoUrl) -> Result<Value, Error> {
        let url = url.into_url()?;
        let response = self.0.get(url.clone()).send().await?;
        let value: serde_json::Value = response.json().await?;
        let mut value = Value::from_json(value)?;
        value.set_href(url);
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use stac::Href;

    #[tokio::test]
    async fn client() {
        let client = Client::new();
        let href = "https://raw.githubusercontent.com/radiantearth/stac-spec/v1.0.0/examples/simple-item.json";
        let value = client.get(href).await.unwrap();
        assert_eq!(value.href().unwrap(), href);
    }
}