pub mod author;
pub mod client;
pub mod client_async;
pub mod date;
pub mod error;
pub mod funding;
pub mod organization;
pub mod publication_date;
pub mod role;
pub mod utils;
pub mod work;

// Re-export public structs for convenience
pub use author::Author;
pub use client::Client;
pub use client_async::AsyncClient;
pub use date::Date;
pub use error::{OrcidError, Result};
pub use funding::Funding;
pub use organization::Organization;
pub use publication_date::PublicationDate;
pub use role::Role;
pub use work::Work;
