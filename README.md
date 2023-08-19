# minip2p

a user-friendly client for p2p

## Usage

```rust
use minip2p::Client;
fn main() {
    let client = Client::new(port);
    client.connect_to_relay(relay_server)?
        .pair_with(code)?;
}


```