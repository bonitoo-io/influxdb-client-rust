# influxdb-client-rust

[![CircleCI](https://circleci.com/gh/bonitoo-io/influxdb-client-rust.svg?style=svg)](https://circleci.com/gh/bonitoo-io/influxdb-client-rust)
[![codecov](https://codecov.io/gh/bonitoo-io/influxdb-client-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/bonitoo-io/influxdb-client-rust)
[![License](https://img.shields.io/github/license/bonitoo-io/influxdb-client-rust.svg)](https://github.com/bonitoo-io/influxdb-client-rust/blob/master/LICENSE)
[![Latest Version](https://img.shields.io/crates/v/influxdb_client_rust.svg)](https://crates.io/crates/influxdb_client_rust)
[![Documentation](https://docs.rs/influxdb_client_rust/badge.svg)](https://docs.rs/influxdb_client_rust)
[![GitHub issues](https://img.shields.io/github/issues-raw/bonitoo-io/influxdb-client-rust.svg)](https://github.com/bonitoo-io/influxdb-client-rust/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr-raw/bonitoo-io/influxdb-client-rust.svg)](https://github.com/bonitoo-io/influxdb-client-rust/pulls)
[![Slack Status](https://img.shields.io/badge/slack-join_chat-white.svg?logo=slack&style=social)](https://www.influxdata.com/slack)

This repository contains the reference Rust client for the InfluxDB 2.0.

#### Disclaimer: This library is a work in progress and should not be considered production ready yet.

## Features
- [Supports InfluxDB 1.8 v2 compatibility endpoints](#influxdb-18-api-compatibility)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
influxdb_client_rust = "1.0.0-alpha"
```

## Usage

### Creating a client

Use **influxdb_client_rust::Client::new** to create a client connected to a running InfluxDB 2 instance authenticate by [token](https://v2.docs.influxdata.com/v2.0/security/tokens/).

```rust
use influxdb_client_rust::Client;
 
let client = Client::new("http://localhost:9999", "my-token");
```

#### Client Options

| Option | Description | Type | Default |
|---|---|---|---|
| bucket | Default destination bucket for writes | String | none |
| org | Default organization bucket for writes | String | none |
| precision | Default precision for the unix timestamps within the body line-protocol | WritePrecision | ns |

##### Configure default `Bucket`, `Organization` and `Precision`

```rust
use influxdb_client_rust::Client;
use influxdb_client_rust::generated::models::WritePrecision;

let client = Client::new("http://localhost:9999", "my-token")
    .with_bucket("my-bucket")
    .with_org("my-org")
    .with_precision(WritePrecision::S);
```

#### InfluxDB 1.8 API compatibility

Use **influxdb_client_rust::Client::new_v1** to create a client connected to InfluxDB 1.8.

```rust
use influxdb_client_rust::Client;

let client = Client::new_v1("http://localhost:8086", 
    "my-user", 
    "my-password", 
    "telegraf", 
    "autogen"
);
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/bonitoo-io/influxdb-client-rust.

## License

The client is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
