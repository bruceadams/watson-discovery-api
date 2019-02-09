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
pub struct QueryFilterType {
  /// A comma-separated list of types to exclude.
  #[serde(rename = "exclude")]
  exclude: Option<Vec<String>>,
  /// A comma-separated list of types to include. All other types are excluded.
  #[serde(rename = "include")]
  include: Option<Vec<String>>
}

impl QueryFilterType {
  pub fn new() -> QueryFilterType {
    QueryFilterType {
      exclude: None,
      include: None
    }
  }

  pub fn set_exclude(&mut self, exclude: Vec<String>) {
    self.exclude = Some(exclude);
  }

  pub fn with_exclude(mut self, exclude: Vec<String>) -> QueryFilterType {
    self.exclude = Some(exclude);
    self
  }

  pub fn exclude(&self) -> Option<&Vec<String>> {
    self.exclude.as_ref()
  }

  pub fn reset_exclude(&mut self) {
    self.exclude = None;
  }

  pub fn set_include(&mut self, include: Vec<String>) {
    self.include = Some(include);
  }

  pub fn with_include(mut self, include: Vec<String>) -> QueryFilterType {
    self.include = Some(include);
    self
  }

  pub fn include(&self) -> Option<&Vec<String>> {
    self.include.as_ref()
  }

  pub fn reset_include(&mut self) {
    self.include = None;
  }

}



