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
    //TODO to generated code
    /// The default precision for the unix timestamps within.
    precision: Option<String>,
    //TODO timeout or client
}

impl Client {
    /// Instantiate a new [`Client`](struct.Client.html).
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
            precision: None,
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
    /// let client = Client::new("http://localhost:9999", "my-token").with_bucket("my-bucket");
    /// ```
    pub fn with_bucket<T>(mut self, bucket: T) -> Self
    where
        T: Into<String>,
    {
        self.bucket = Some(bucket.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[test]
    fn test_default_values() {
        let client = Client::new("http://localhost:9999", "my-token");
        assert_eq!(client.org, None);
        assert_eq!(client.bucket, None);
        assert_eq!(client.precision, None);
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
}
