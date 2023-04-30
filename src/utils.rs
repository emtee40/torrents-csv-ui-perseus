use crate::types::torrent::Torrent;
use chrono::NaiveDateTime;

pub fn pretty_date(unix: u32) -> String {
  let naive = NaiveDateTime::from_timestamp_opt(unix.into(), 0);
  naive
    .map(|d| d.format("%Y-%m-%d").to_string())
    .unwrap_or("Now".to_string())
}

pub fn magnet_link(torrent: &Torrent) -> String {
  format!(
    "magnet:?xt=urn:btih:{}&dn={}",
    torrent.infohash, torrent.name
  )
}
