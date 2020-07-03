use crate::generated::models::write_precision::WritePrecision;
use reqwest::{header, Client as HttpClient, IntoUrl, RequestBuilder, Url};

/// Authorization information
#[derive(Clone, Debug)]
struct Auth {
    /// Access Token used for authenticating/authorizing the InfluxDB request sent by client.
    authorization_header: Option<String>,
}

/// The client is the entry point to HTTP API defined
/// in https://github.com/influxdata/influxdb/blob/master/http/swagger.yml.
#[derive(Clone, Debug)]
pub struct Client {
    /// InfluxDB URL to connect to (ex. `http://localhost:9999`).
    url: Url,
    /// Authorization information.
    auth: Auth,
    /// The default organization bucket for writes.
    org: Option<String>,
    /// The default destination bucket for writes.
    bucket: Option<String>,
    /// The default precision for the unix timestamps within.
    precision: Option<WritePrecision>,
    /// The HTTP client.
    http_client: HttpClient,
}

impl Client {
    /// Instantiate a new [`Client`](struct.Client.html). The default client precision is [`WritePrecision::Ns`](generated/models/write_precision/enum.WritePrecision.html#variant.Ns).
    ///
    /// # Arguments
    ///
    /// * `url` - InfluxDB URL to connect to (ex. `http://localhost:9999`).
    /// * `token` - Access Token used for authenticating/authorizing the InfluxDB request sent by client.
    ///
    /// # Examples
    ///
    /// ```
    /// use influxdb_client_rust::Client;
    ///
    /// let client = Client::new("http://localhost:9999", "my-token");
    /// ```
    ///
    /// ```
    /// use influxdb_client_rust::Client;
    /// use influxdb_client_rust::generated::models::WritePrecision;
    ///
    /// let client = Client::new("http://localhost:9999", "my-token")
    ///     .with_bucket("my-bucket")
    ///     .with_org("my-org")
    ///     .with_precision(WritePrecision::S);
    /// ```
    pub fn new<T1, T2>(url: T1, token: T2) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        let url = match Url::parse(&url.into()) {
            Ok(url) => url,
            Err(err) => panic!(format!("{}", err)),
        };

        let auth = Auth {
            authorization_header: Some(format!("Token {}", &token.into())),
        };

        Client {
            url,
            auth,
            org: None,
            bucket: None,
            precision: Some(WritePrecision::Ns),
            http_client: Client::build_http_client(),
        }
    }

    /// Add org to [`Client`](struct.Client.html).
    ///
    /// # Arguments
    ///
    /// * `org` - The default organization bucket for writes.
    ///
    /// # Example
    ///
    /// ```
    /// use influxdb_client_rust::Client;
    ///
    /// let client = Client::new("http://localhost:9999", "my-token").with_org("my-org");
    /// ```
    pub fn with_org<S>(mut self, org: S) -> Self
    where
        S: Into<String>,
    {
        self.org = Some(org.into());
        self
    }

    /// Add bucket to [`Client`](struct.Client.html).
    ///
    /// # Arguments
    ///
    /// * `bucket` - The default destination bucket for writes.
    ///
    /// # Example
    ///
    /// ```
    /// use influxdb_client_rust::Client;
    ///
    /// let client = Client::new("http://localhost:9999", "my-token").with_bucket("my-bucket");
    /// ```
    pub fn with_bucket<S>(mut self, bucket: S) -> Self
    where
        S: Into<String>,
    {
        self.bucket = Some(bucket.into());
        self
    }

    /// Add default precision to [`Client`](struct.Client.html).
    ///
    /// # Arguments
    ///
    /// * `precision` - The default precision for the unix timestamps within [`WritePrecision`](generated/models/write_precision/enum.WritePrecision.html).
    ///
    /// # Example
    ///
    /// ```
    /// use influxdb_client_rust::Client;
    /// use influxdb_client_rust::generated::models::WritePrecision;
    ///
    /// let client = Client::new("http://localhost:9999", "my-token").with_precision(WritePrecision::S);
    /// ```
    pub fn with_precision<WP>(mut self, precision: WP) -> Self
    where
        WP: Into<WritePrecision>,
    {
        self.precision = Some(precision.into());
        self
    }

    ///TODO
    #[allow(dead_code)]
    fn build_request<IU>(&self, url: IU, method: reqwest::Method) -> RequestBuilder
    where
        IU: IntoUrl,
    {
        let request = self.http_client.request(method, url);

        if let Some(i) = &self.auth.authorization_header {
            request.header(header::AUTHORIZATION, i)
        } else {
            request
        }
    }

    fn build_http_client() -> HttpClient {
        const APP_USER_AGENT: &str = concat!("influxdb-client-rust/", env!("CARGO_PKG_VERSION"));

        reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::generated::models::write_precision::WritePrecision;
    use crate::Client;
    use reqwest::{Method, Url};
    extern crate httpmock;

    use httpmock::Method::GET;
    use httpmock::{mock, with_mock_server};

    #[test]
    fn test_default_values() {
        let client = Client::new("http://localhost:9999", "my-token");
        assert_eq!(client.org, None);
        assert_eq!(client.bucket, None);
        assert!(client.precision.is_some());
        assert_eq!(client.precision.unwrap(), WritePrecision::Ns);
    }

    #[test]
    fn test_url() {
        let client = Client::new("http://localhost:9999", "my-token");
        assert_eq!(client.url, Url::parse("http://localhost:9999").unwrap());
    }

    #[test]
    #[should_panic(expected = "relative URL without a base")]
    fn test_url_panic() {
        Client::new("xyz", "my-token");
    }

    #[test]
    fn test_token() {
        let client = Client::new("http://localhost:9999", "my-token");
        assert!(client.auth.authorization_header.is_some());
        assert_eq!(client.auth.authorization_header.unwrap(), "Token my-token");
    }

    #[test]
    fn test_org() {
        let client = Client::new("http://localhost:9999", "my-token").with_org("my-org");
        assert!(client.org.is_some());
        assert_eq!(client.org.unwrap(), "my-org");
    }

    #[test]
    fn test_bucket() {
        let client = Client::new("http://localhost:9999", "my-token").with_bucket("my-bucket");
        assert!(client.bucket.is_some());
        assert_eq!(client.bucket.unwrap(), "my-bucket");
    }

    #[test]
    fn test_precision() {
        let client =
            Client::new("http://localhost:9999", "my-token").with_precision(WritePrecision::S);
        assert!(client.precision.is_some());
        assert_eq!(client.precision.unwrap(), WritePrecision::S);
    }

    #[tokio::test]
    #[with_mock_server]
    async fn test_user_agent_header() {
        let mock = mock(GET, "/api/v2/mock")
            .expect_header("User-Agent", "influxdb-client-rust/1.0.0-alpha")
            .return_status(204)
            .create();

        let response = Client::new("http://localhost:5000", "my-token")
            .build_request(mock_url(), Method::GET)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 204);
        assert_eq!(mock.times_called(), 1);
    }

    #[tokio::test]
    #[with_mock_server]
    async fn test_token_header() {
        let mock = mock(GET, "/api/v2/mock")
            .expect_header("Authorization", "Token my-token")
            .return_status(204)
            .create();

        let response = Client::new("http://localhost:5000", "my-token")
            .build_request(mock_url(), Method::GET)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 204);
        assert_eq!(mock.times_called(), 1);
    }

    fn mock_url() -> Url {
        Url::parse("http://localhost:5000/api/v2/mock").unwrap()
    }
}
