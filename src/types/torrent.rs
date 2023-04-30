use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Torrent {
  pub infohash: String,
  pub name: String,
  pub size_bytes: u64,
  pub created_unix: u32,
  pub seeders: u32,
  pub leechers: u32,
  pub completed: Option<u32>,
  pub scraped_date: u32,
}
