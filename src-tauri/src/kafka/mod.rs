pub mod client;
pub mod mock;

pub use client::KafkaClient;

#[cfg(test)]
mod tests;
