/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CollectionDiskUsage : Summary of the disk usage statistics for this collection.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionDiskUsage {
  /// Number of bytes used by the collection.
  #[serde(rename = "used_bytes")]
  used_bytes: Option<i32>
}

impl CollectionDiskUsage {
  /// Summary of the disk usage statistics for this collection.
  pub fn new() -> CollectionDiskUsage {
    CollectionDiskUsage {
      used_bytes: None
    }
  }

  pub fn set_used_bytes(&mut self, used_bytes: i32) {
    self.used_bytes = Some(used_bytes);
  }

  pub fn with_used_bytes(mut self, used_bytes: i32) -> CollectionDiskUsage {
    self.used_bytes = Some(used_bytes);
    self
  }

  pub fn used_bytes(&self) -> Option<&i32> {
    self.used_bytes.as_ref()
  }

  pub fn reset_used_bytes(&mut self) {
    self.used_bytes = None;
  }

}


