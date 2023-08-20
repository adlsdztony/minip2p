# minip2p

a simple rust library for p2p communication

## Usage

```rust
use minip2p::Client;
fn main() {
    let client = Client::new(port);
    client.connect_to_relay(relay_server)?
        .pair_with(code)?;
}

```

## examples
### serverp2p

a simple command line p2p server for minip2p

```shell
# clone this repository
git clone https://github.com/adlsdztony/minip2p

# build with cargo
cd minip2p
cargo build --release --example serverp2p

# run
./target/release/examples/serverp2p -p <port>

```
### chatp2p

a command line p2p chat application

```shell
# clone this repository
git clone https://github.com/adlsdztony/minip2p

# build with cargo
cd minip2p
cargo build --release --example chatp2p

# run
./target/release/examples/chatp2p -r <relay server (ip:port)> -c <pair code> -p <local port>

```