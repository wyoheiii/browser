
use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
  url: String,
  host: String,
  port: String,
  path: String,
  searchpart: String,
}

use alloc::string::ToString;
use alloc::vec::Vec;

impl Url {
  pub fn new(url: String) -> Self {
    Self {
      url,
      host: String::new(),
      port: String::new(),
      path: String::new(),
      searchpart: String::new(),
    }
  }

  pub fn parse(&mut self) -> Result<Self, String> {
    if !self.is_http() {
      return Err("only HTTP scheme is supported.".to_string());
    }

    self.host = self.extract_host();
    self.port = self.extract_port();
    self.path = self.extract_path();
    self.searchpart = self.extract_searchpart();

    Ok(self.clone())
  }

  pub fn host(&self) -> String {
    self.host.clone()
  }

  pub fn port(&self) -> String {
    self.port.clone()
  }

  pub fn path(&self) -> String {
    self.path.clone()
  }

  pub fn searchpart(&self) -> String {
    self.searchpart.clone()
  }


  fn is_http(&self) -> bool {
    // URL の構文を定めた標準規格（RFC 3986）に従い、スキームは必ず URL の最初に来るべきものであり、途中にスキームが現れることはありません。
    self.url.starts_with("http://")
  }

  fn extract_host(&self) -> String {
    let url_parts: Vec<&str> = self
    .url
    .trim_start_matches("http://")
    .splitn(2, "/")
    .collect();

    let host:&str = url_parts[0];

    match host.find(':') {
      Some(index) => host[..index].to_string(),
      None => host.to_string(),
    }
  }

  fn extract_port(&self) -> String {
    let url_parts: Vec<&str> = self
    .url
    .trim_start_matches("http://")
    .splitn(2, "/")
    .collect();

    let host:&str = url_parts[0];

    match host.find(':') {
      Some(index) => host[index + 1..].to_string(),
      None => "80".to_string(),
    }
  }

  fn extract_path(&self) -> String {
    let url_parts: Vec<&str> = self
    .url
    .trim_start_matches("http://")
    .splitn(2, "/")
    .collect();

    if url_parts.len() < 2 {
      return String::new();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();

    path_and_searchpart[0].to_string()
  }

  fn extract_searchpart(&self)-> String {
    let url_parts: Vec<&str> = self
    .url
    .trim_start_matches("http://")
    .splitn(2, "/")
    .collect();

    if url_parts.len() < 2 {
      return String::new();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();

    path_and_searchpart.get(1).unwrap_or(&"").to_string()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_url_host() {
    let url = Url::new("http://example.com".to_string()).parse().unwrap();
    assert_eq!(url.host(), "example.com".to_string());
  }

  #[test]
  fn test_url_host_port() {
    let url = Url::new("http://example.com:8080".to_string()).parse().unwrap();
    assert_eq!(url.host(), "example.com".to_string());
    assert_eq!(url.port(), "8080".to_string());
  }

  #[test]
  fn test_url_host_port_path() {
    let url = Url::new("http://example.com:8080/path".to_string()).parse().unwrap();
    assert_eq!(url.host(), "example.com".to_string());
    assert_eq!(url.port(), "8080".to_string());
    assert_eq!(url.path(), "path".to_string());
  }

  #[test]
  fn test_url_host_port_path_searchpart() {
    let url = Url::new("http://example.com:8080/path?a=123&b=456".to_string()).parse().unwrap();
    assert_eq!(url.host(), "example.com".to_string());
    assert_eq!(url.port(), "8080".to_string());
    assert_eq!(url.path(), "path".to_string());
    assert_eq!(url.searchpart(), "a=123&b=456".to_string());
  }

  #[test]
  fn test_no_scheme() {
    let url = Url::new("example.com".to_string()).parse();
    assert_eq!(url, Err("only HTTP scheme is supported.".to_string()));
  }

  #[test]
  fn test_unsupported_scheme() {
    let url = Url::new("https://example.com".to_string()).parse();
    assert_eq!(url, Err("only HTTP scheme is supported.".to_string()));
  }

}
