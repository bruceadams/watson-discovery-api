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
pub struct QueryResult {
  /// The unique identifier of the document.
  #[serde(rename = "id")]
  id: Option<String>,
  /// *Deprecated* This field is now part of the **result_metadata** object.
  #[serde(rename = "score")]
  score: Option<f64>,
  /// Metadata of the document.
  #[serde(rename = "metadata")]
  metadata: Option<Value>,
  /// The collection ID of the collection containing the document for this result.
  #[serde(rename = "collection_id")]
  collection_id: Option<String>,
  #[serde(rename = "result_metadata")]
  result_metadata: Option<::models::QueryResultMetadata>
}

impl QueryResult {
  pub fn new() -> QueryResult {
    QueryResult {
      id: None,
      score: None,
      metadata: None,
      collection_id: None,
      result_metadata: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> QueryResult {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_score(&mut self, score: f64) {
    self.score = Some(score);
  }

  pub fn with_score(mut self, score: f64) -> QueryResult {
    self.score = Some(score);
    self
  }

  pub fn score(&self) -> Option<&f64> {
    self.score.as_ref()
  }

  pub fn reset_score(&mut self) {
    self.score = None;
  }

  pub fn set_metadata(&mut self, metadata: Value) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: Value) -> QueryResult {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&Value> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_collection_id(&mut self, collection_id: String) {
    self.collection_id = Some(collection_id);
  }

  pub fn with_collection_id(mut self, collection_id: String) -> QueryResult {
    self.collection_id = Some(collection_id);
    self
  }

  pub fn collection_id(&self) -> Option<&String> {
    self.collection_id.as_ref()
  }

  pub fn reset_collection_id(&mut self) {
    self.collection_id = None;
  }

  pub fn set_result_metadata(&mut self, result_metadata: ::models::QueryResultMetadata) {
    self.result_metadata = Some(result_metadata);
  }

  pub fn with_result_metadata(mut self, result_metadata: ::models::QueryResultMetadata) -> QueryResult {
    self.result_metadata = Some(result_metadata);
    self
  }

  pub fn result_metadata(&self) -> Option<&::models::QueryResultMetadata> {
    self.result_metadata.as_ref()
  }

  pub fn reset_result_metadata(&mut self) {
    self.result_metadata = None;
  }

}


