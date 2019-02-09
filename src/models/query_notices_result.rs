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
pub struct QueryNoticesResult {
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
  result_metadata: Option<::models::QueryResultMetadata>,
  /// The internal status code returned by the ingestion subsystem indicating the overall result of ingesting the source document.
  #[serde(rename = "code")]
  code: Option<i32>,
  /// Name of the original source file (if available).
  #[serde(rename = "filename")]
  filename: Option<String>,
  /// The type of the original source file.
  #[serde(rename = "file_type")]
  file_type: Option<String>,
  /// The SHA-1 hash of the original source file (formatted as a hexadecimal string).
  #[serde(rename = "sha1")]
  sha1: Option<String>,
  /// Array of notices for the document.
  #[serde(rename = "notices")]
  notices: Option<Vec<::models::Notice>>
}

impl QueryNoticesResult {
  pub fn new() -> QueryNoticesResult {
    QueryNoticesResult {
      id: None,
      score: None,
      metadata: None,
      collection_id: None,
      result_metadata: None,
      code: None,
      filename: None,
      file_type: None,
      sha1: None,
      notices: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> QueryNoticesResult {
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

  pub fn with_score(mut self, score: f64) -> QueryNoticesResult {
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

  pub fn with_metadata(mut self, metadata: Value) -> QueryNoticesResult {
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

  pub fn with_collection_id(mut self, collection_id: String) -> QueryNoticesResult {
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

  pub fn with_result_metadata(mut self, result_metadata: ::models::QueryResultMetadata) -> QueryNoticesResult {
    self.result_metadata = Some(result_metadata);
    self
  }

  pub fn result_metadata(&self) -> Option<&::models::QueryResultMetadata> {
    self.result_metadata.as_ref()
  }

  pub fn reset_result_metadata(&mut self) {
    self.result_metadata = None;
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: i32) -> QueryNoticesResult {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&i32> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

  pub fn set_filename(&mut self, filename: String) {
    self.filename = Some(filename);
  }

  pub fn with_filename(mut self, filename: String) -> QueryNoticesResult {
    self.filename = Some(filename);
    self
  }

  pub fn filename(&self) -> Option<&String> {
    self.filename.as_ref()
  }

  pub fn reset_filename(&mut self) {
    self.filename = None;
  }

  pub fn set_file_type(&mut self, file_type: String) {
    self.file_type = Some(file_type);
  }

  pub fn with_file_type(mut self, file_type: String) -> QueryNoticesResult {
    self.file_type = Some(file_type);
    self
  }

  pub fn file_type(&self) -> Option<&String> {
    self.file_type.as_ref()
  }

  pub fn reset_file_type(&mut self) {
    self.file_type = None;
  }

  pub fn set_sha1(&mut self, sha1: String) {
    self.sha1 = Some(sha1);
  }

  pub fn with_sha1(mut self, sha1: String) -> QueryNoticesResult {
    self.sha1 = Some(sha1);
    self
  }

  pub fn sha1(&self) -> Option<&String> {
    self.sha1.as_ref()
  }

  pub fn reset_sha1(&mut self) {
    self.sha1 = None;
  }

  pub fn set_notices(&mut self, notices: Vec<::models::Notice>) {
    self.notices = Some(notices);
  }

  pub fn with_notices(mut self, notices: Vec<::models::Notice>) -> QueryNoticesResult {
    self.notices = Some(notices);
    self
  }

  pub fn notices(&self) -> Option<&Vec<::models::Notice>> {
    self.notices.as_ref()
  }

  pub fn reset_notices(&mut self) {
    self.notices = None;
  }

}



