pub mod author;
pub mod client;
pub mod date;
pub mod organization;
pub mod publication_date;
pub mod role;
pub mod work;

// Re-export public structs for convenience
pub use author::Author;
pub use client::Client;
pub use date::Date;
pub use organization::Organization;
pub use publication_date::PublicationDate;
pub use role::Role;
pub use work::Work;
