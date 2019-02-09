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
pub struct ListCollectionsResponse {
  /// An array containing information about each collection in the environment.
  #[serde(rename = "collections")]
  collections: Option<Vec<::models::Collection>>
}

impl ListCollectionsResponse {
  pub fn new() -> ListCollectionsResponse {
    ListCollectionsResponse {
      collections: None
    }
  }

  pub fn set_collections(&mut self, collections: Vec<::models::Collection>) {
    self.collections = Some(collections);
  }

  pub fn with_collections(mut self, collections: Vec<::models::Collection>) -> ListCollectionsResponse {
    self.collections = Some(collections);
    self
  }

  pub fn collections(&self) -> Option<&Vec<::models::Collection>> {
    self.collections.as_ref()
  }

  pub fn reset_collections(&mut self) {
    self.collections = None;
  }

}



