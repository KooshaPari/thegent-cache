# Hello World Story

<StoryHeader
    title="Your First thegent-cache Operation"
    :duration="2"
    :gif="'/gifs/thegent-cache-hello-world.gif'"
    difficulty="beginner"
/>

## Objective

Get thegent-cache running with a basic operation.

## Prerequisites

- Rust/Node/Python installed
- thegent-cache package installed

## Implementation

```rust
use thegent-cache::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::new().await?;
    
    // Execute operation
    let result = client.hello().await?;
    
    println!("Success: {}", result);
    Ok(())
}
```

## Expected Output

```
Success: Hello from thegent-cache!
```

## Next Steps

- [Core Integration](./core-integration)
- Read [API Reference](../reference/api)
