# wmata
wmata is a lightweight Rust interface to the [Washington Metropolitan Area Transit Authority API](https://developer.wmata.com).

## Contents
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
    - [Getting Started](#getting-started)
    - [Design](#design)
    - [Using `RailClient`](#using-railclient)
    - [Using `BusClient`](#using-busclient)
- [Testing](#testing)
- [Dependencies](#dependencies)
- [Contact](#contact)
- [License](#license)

## Requirements
- Rust 2018

## Installation

### Cargo
```toml
wmata = "2.0.1"
```

## Usage

### Getting Started
```rust
use wmata::RailClient;

let client = RailClient::new(api_key);

let trains = client.next_trains()?;
```

### Design
wmata breaks the WMATA API into two components: `RailClient` and `BusClient`.

#### `RailClient`
Provides access to all MetroRail related endpoints.

##### Using `RailClient`
```rust
use wmata::RailClient;

let client = RailClient::new(api_key);

let trains = client.next_trains()?;
```

#### `BusClient`
Provides access to all MetroBus related endpoints.


##### Using `BusClient`
```rust
use wmata::BusClient;

let client = BusClient::new(api_key);

let routes = client.routes()?;
```

## Testing
Note that tests must currently be run with `--test-threads 1` in order to pass, due to using live data.

## Dependencies
- serde
- serde_json
- reqwest

## Contact
Feel free to email questions and comments to [emma@emma.sh](mailto:emma@emma.sh)

## License

wmata is released under the MIT license. [See LICENSE](https://github.com/emma-foster/wmata/blob/master/LICENSE) for details.