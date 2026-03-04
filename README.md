# ORCID Rust Library

A Rust library for interacting with the ORCID API to fetch researcher information.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
orcid = "0.2"
```

## Usage

### Async Client (Default)

```rust
use orcid::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    
    // Get author information
    let author = client.author("0000-0001-5916-0947").await?;
    
    println!("Name: {}", author.full_name().unwrap_or_default());
    println!("ORCID: {}", author.orcid_id().unwrap_or_default());
    
    // Search for researchers
    let results = client.search("climate change").await?;
    println!("Found {} researchers", results.len());
    
    Ok(())
}
```

### Blocking Client

```rust
use orcid::{ClientBlocking, Result};

fn main() -> Result<()> {
    let client = ClientBlocking::new();
    
    // Get author information
    let author = client.author("0000-0001-5916-0947")?;
    
    // Access various data
    println!("Works: {:?}", author.works());
    println!("Education: {:?}", author.education());
    println!("Employment: {:?}", author.employment());
    
    Ok(())
}
```

### Search Builder

Build complex search queries easily:

```rust
let results = client.search_builder()
    .with_keyword("machine learning")
    .with_affiliation("MIT")
    .limit(50)
    .execute()
    .await?;
```

## Features

- Fetch complete ORCID profiles including:
  - Personal information (names, biography, keywords)
  - Works and publications
  - Education history
  - Employment records
  - Funding information
  - Peer reviews
  - Memberships
  - Qualifications
- Search for researchers by keywords, DOI, affiliation, etc.
- Validate ORCID IDs
- Both async and blocking API clients

## License

MIT License - see LICENSE file for details