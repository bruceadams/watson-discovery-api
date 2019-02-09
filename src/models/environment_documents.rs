/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnvironmentDocuments : Summary of the document usage statistics for the environment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentDocuments {
  /// Number of documents indexed for the environment.
  #[serde(rename = "indexed")]
  indexed: Option<i32>,
  /// Total number of documents allowed in the environment's capacity.
  #[serde(rename = "maximum_allowed")]
  maximum_allowed: Option<i32>
}

impl EnvironmentDocuments {
  /// Summary of the document usage statistics for the environment.
  pub fn new() -> EnvironmentDocuments {
    EnvironmentDocuments {
      indexed: None,
      maximum_allowed: None
    }
  }

  pub fn set_indexed(&mut self, indexed: i32) {
    self.indexed = Some(indexed);
  }

  pub fn with_indexed(mut self, indexed: i32) -> EnvironmentDocuments {
    self.indexed = Some(indexed);
    self
  }

  pub fn indexed(&self) -> Option<&i32> {
    self.indexed.as_ref()
  }

  pub fn reset_indexed(&mut self) {
    self.indexed = None;
  }

  pub fn set_maximum_allowed(&mut self, maximum_allowed: i32) {
    self.maximum_allowed = Some(maximum_allowed);
  }

  pub fn with_maximum_allowed(mut self, maximum_allowed: i32) -> EnvironmentDocuments {
    self.maximum_allowed = Some(maximum_allowed);
    self
  }

  pub fn maximum_allowed(&self) -> Option<&i32> {
    self.maximum_allowed.as_ref()
  }

  pub fn reset_maximum_allowed(&mut self) {
    self.maximum_allowed = None;
  }

}



