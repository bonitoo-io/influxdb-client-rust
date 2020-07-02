use crate::generated::models::write_precision::WritePrecision;

/// The client is the entry point to HTTP API defined
/// in https://github.com/influxdata/influxdb/blob/master/http/swagger.yml.
#[derive(Clone, Debug)]
pub struct Client {
    /// InfluxDB URL to connect to (ex. `http://localhost:9999`).
    url: String,
    /// Access Token used for authenticating/authorizing the InfluxDB request sent by client.
    token: String,
    /// The default organization bucket for writes.
    org: Option<String>,
    /// The default destination bucket for writes.
    bucket: Option<String>,
    /// The default precision for the unix timestamps within.
    precision: Option<WritePrecision>,
    /// The HTTP client.
    http_client: reqwest::Client,
}

impl Client {
    /// Instantiate a new [`Client`](struct.Client.html). The default client precision is [`WritePrecision::Ns`](generated/models/write_precision/enum.WritePrecision.html#variant.Ns).
    ///
    /// # Arguments
    ///
    /// * `url` - InfluxDB URL to connect to (ex. `http://localhost:9999`).
    /// * `token` - Access Token used for authenticating/authorizing the InfluxDB request sent by client.
    ///
    /// # Example
    ///
    /// ```
    /// use influxdb_client_rust::Client;
    ///
    /// let client = Client::new("http://localhost:9999", "my-token");
    /// ```
    pub fn new<T1, T2>(url: T1, token: T2) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        Client {
            url: url.into(),
            token: token.into(),
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
    pub fn with_org<T>(mut self, org: T) -> Self
    where
        T: Into<String>,
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
    pub fn with_bucket<T>(mut self, bucket: T) -> Self
    where
        T: Into<String>,
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
    pub fn with_precision<T>(mut self, precision: T) -> Self
    where
        T: Into<WritePrecision>,
    {
        self.precision = Some(precision.into());
        self
    }

    fn build_http_client() -> reqwest::Client {
        static APP_USER_AGENT: &str = concat!("influxdb-client-rust/", env!("CARGO_PKG_VERSION"));

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
        assert_eq!(client.url, "http://localhost:9999");
    }

    #[test]
    fn test_token() {
        let client = Client::new("http://localhost:9999", "my-token");
        assert_eq!(client.token, "my-token");
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
}
