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
pub struct QueryRelationsEntity {
  /// Entity text content.
  #[serde(rename = "text")]
  text: Option<String>,
  /// The type of the specified entity.
  #[serde(rename = "type")]
  type_: Option<String>,
  /// If false, implicit querying is performed. The default is `false`.
  #[serde(rename = "exact")]
  exact: Option<bool>
}

impl QueryRelationsEntity {
  pub fn new() -> QueryRelationsEntity {
    QueryRelationsEntity {
      text: None,
      type_: None,
      exact: None
    }
  }

  pub fn set_text(&mut self, text: String) {
    self.text = Some(text);
  }

  pub fn with_text(mut self, text: String) -> QueryRelationsEntity {
    self.text = Some(text);
    self
  }

  pub fn text(&self) -> Option<&String> {
    self.text.as_ref()
  }

  pub fn reset_text(&mut self) {
    self.text = None;
  }

  pub fn set_type_(&mut self, type_: String) {
    self.type_ = Some(type_);
  }

  pub fn with_type_(mut self, type_: String) -> QueryRelationsEntity {
    self.type_ = Some(type_);
    self
  }

  pub fn type_(&self) -> Option<&String> {
    self.type_.as_ref()
  }

  pub fn reset_type_(&mut self) {
    self.type_ = None;
  }

  pub fn set_exact(&mut self, exact: bool) {
    self.exact = Some(exact);
  }

  pub fn with_exact(mut self, exact: bool) -> QueryRelationsEntity {
    self.exact = Some(exact);
    self
  }

  pub fn exact(&self) -> Option<&bool> {
    self.exact.as_ref()
  }

  pub fn reset_exact(&mut self) {
    self.exact = None;
  }

}


