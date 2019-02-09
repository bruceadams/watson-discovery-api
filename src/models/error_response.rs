/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
  /// The HTTP error status code.
  #[serde(rename = "code")]
  code: i32,
  /// A message describing the error.
  #[serde(rename = "error")]
  error: String
}

impl ErrorResponse {
  pub fn new(code: i32, error: String) -> ErrorResponse {
    ErrorResponse {
      code: code,
      error: error
    }
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = code;
  }

  pub fn with_code(mut self, code: i32) -> ErrorResponse {
    self.code = code;
    self
  }

  pub fn code(&self) -> &i32 {
    &self.code
  }


  pub fn set_error(&mut self, error: String) {
    self.error = error;
  }

  pub fn with_error(mut self, error: String) -> ErrorResponse {
    self.error = error;
    self
  }

  pub fn error(&self) -> &String {
    &self.error
  }


}



