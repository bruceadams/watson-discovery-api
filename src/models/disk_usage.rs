/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DiskUsage : Summary of the disk usage statistics for the environment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskUsage {
  /// Number of bytes within the environment's disk capacity that are currently used to store data.
  #[serde(rename = "used_bytes")]
  used_bytes: Option<i32>,
  /// Total number of bytes available in the environment's disk capacity.
  #[serde(rename = "maximum_allowed_bytes")]
  maximum_allowed_bytes: Option<i32>,
  /// **Deprecated**: Total number of bytes available in the environment's disk capacity.
  #[serde(rename = "total_bytes")]
  total_bytes: Option<i32>,
  /// **Deprecated**: Amount of disk capacity used, in KB or GB format.
  #[serde(rename = "used")]
  used: Option<String>,
  /// **Deprecated**: Total amount of the environment's disk capacity, in KB or GB format.
  #[serde(rename = "total")]
  total: Option<String>,
  /// **Deprecated**: Percentage of the environment's disk capacity that is being used.
  #[serde(rename = "percent_used")]
  percent_used: Option<f64>
}

impl DiskUsage {
  /// Summary of the disk usage statistics for the environment.
  pub fn new() -> DiskUsage {
    DiskUsage {
      used_bytes: None,
      maximum_allowed_bytes: None,
      total_bytes: None,
      used: None,
      total: None,
      percent_used: None
    }
  }

  pub fn set_used_bytes(&mut self, used_bytes: i32) {
    self.used_bytes = Some(used_bytes);
  }

  pub fn with_used_bytes(mut self, used_bytes: i32) -> DiskUsage {
    self.used_bytes = Some(used_bytes);
    self
  }

  pub fn used_bytes(&self) -> Option<&i32> {
    self.used_bytes.as_ref()
  }

  pub fn reset_used_bytes(&mut self) {
    self.used_bytes = None;
  }

  pub fn set_maximum_allowed_bytes(&mut self, maximum_allowed_bytes: i32) {
    self.maximum_allowed_bytes = Some(maximum_allowed_bytes);
  }

  pub fn with_maximum_allowed_bytes(mut self, maximum_allowed_bytes: i32) -> DiskUsage {
    self.maximum_allowed_bytes = Some(maximum_allowed_bytes);
    self
  }

  pub fn maximum_allowed_bytes(&self) -> Option<&i32> {
    self.maximum_allowed_bytes.as_ref()
  }

  pub fn reset_maximum_allowed_bytes(&mut self) {
    self.maximum_allowed_bytes = None;
  }

  pub fn set_total_bytes(&mut self, total_bytes: i32) {
    self.total_bytes = Some(total_bytes);
  }

  pub fn with_total_bytes(mut self, total_bytes: i32) -> DiskUsage {
    self.total_bytes = Some(total_bytes);
    self
  }

  pub fn total_bytes(&self) -> Option<&i32> {
    self.total_bytes.as_ref()
  }

  pub fn reset_total_bytes(&mut self) {
    self.total_bytes = None;
  }

  pub fn set_used(&mut self, used: String) {
    self.used = Some(used);
  }

  pub fn with_used(mut self, used: String) -> DiskUsage {
    self.used = Some(used);
    self
  }

  pub fn used(&self) -> Option<&String> {
    self.used.as_ref()
  }

  pub fn reset_used(&mut self) {
    self.used = None;
  }

  pub fn set_total(&mut self, total: String) {
    self.total = Some(total);
  }

  pub fn with_total(mut self, total: String) -> DiskUsage {
    self.total = Some(total);
    self
  }

  pub fn total(&self) -> Option<&String> {
    self.total.as_ref()
  }

  pub fn reset_total(&mut self) {
    self.total = None;
  }

  pub fn set_percent_used(&mut self, percent_used: f64) {
    self.percent_used = Some(percent_used);
  }

  pub fn with_percent_used(mut self, percent_used: f64) -> DiskUsage {
    self.percent_used = Some(percent_used);
    self
  }

  pub fn percent_used(&self) -> Option<&f64> {
    self.percent_used.as_ref()
  }

  pub fn reset_percent_used(&mut self) {
    self.percent_used = None;
  }

}


