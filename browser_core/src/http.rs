use alloc::string::String;
use alloc::vec::Vec;
use crate::error::Error;


#[derive(Debug, Clone)]
pub struct Header {
  name: String,
  value: String,
}

impl Header {
  pub fn new(name: String, value: String) -> Self {
    Self {
      name,
      value,
    }
  }
}

#[derive(Debug, Clone)]
pub struct httpResponse {
  version: String,
  status_code: u32,
  reason: String,
  headers: Vec<Header>,
  body: String,
}

impl httpResponse {
  pub fn new(raw_response: String) -> Result<Self, Error> {
    panic!("Not implemented");
  }


}