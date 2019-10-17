# wmata
wmata is a high level Rust interface to the [Washington Metropolitan Area Transit Authority API](https://developer.wmata.com).

## Contents
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
    - [Getting Started](#getting-started)
    - [Design](#design)
    - [Using `MetroRail`](#using-MetroRail)
    - [Using `MetroBus`](#using-MetroBus)
- [Testing](#testing)
- [Dependencies](#dependencies)
- [Contact](#contact)
- [License](#license)

## Requirements
- Rust 2018

## Installation

### Cargo
```toml
wmata = "5.0.0"
```

## Usage

### Getting Started
```rust
use wmata::{MetroRail, Station};

let client = MetroRail::new(api_key);

let trains = client.next_trains(Station::A01)?;
```

### Design
wmata breaks the WMATA API into two components: `MetroRail` and `MetroBus`.

#### `MetroRail`
Provides access to all MetroRail related endpoints.

##### Using `MetroRail`
```rust
use wmata::{MetroRail, Station};

let client = MetroRail::new(api_key);

let trains = client.next_trains(Station::A01)?;
```

#### `MetroBus`
Provides access to all MetroBus related endpoints.


##### Using `MetroBus`
```rust
use wmata::MetroBus;

let client = MetroBus::new(api_key);

let routes = client.routes()?;
```

## Testing
Note that tests must currently be run with `--test-threads 1` in order to pass, due to using live data.

## Dependencies
- serde
- serde_json
- reqwest
- chrono

## Contact
Feel free to email questions and comments to [emma@emma.sh](mailto:emma@emma.sh)

## License

wmata is released under the MIT license. [See LICENSE](https://github.com/emma-foster/wmata/blob/master/LICENSE) for details.