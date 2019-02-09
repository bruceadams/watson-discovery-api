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
pub struct QueryPassages {
  /// The unique identifier of the document from which the passage has been extracted.
  #[serde(rename = "document_id")]
  document_id: Option<String>,
  /// The confidence score of the passages's analysis. A higher score indicates greater confidence.
  #[serde(rename = "passage_score")]
  passage_score: Option<f64>,
  /// The content of the extracted passage.
  #[serde(rename = "passage_text")]
  passage_text: Option<String>,
  /// The position of the first character of the extracted passage in the originating field.
  #[serde(rename = "start_offset")]
  start_offset: Option<i32>,
  /// The position of the last character of the extracted passage in the originating field.
  #[serde(rename = "end_offset")]
  end_offset: Option<i32>,
  /// The label of the field from which the passage has been extracted.
  #[serde(rename = "field")]
  field: Option<String>
}

impl QueryPassages {
  pub fn new() -> QueryPassages {
    QueryPassages {
      document_id: None,
      passage_score: None,
      passage_text: None,
      start_offset: None,
      end_offset: None,
      field: None
    }
  }

  pub fn set_document_id(&mut self, document_id: String) {
    self.document_id = Some(document_id);
  }

  pub fn with_document_id(mut self, document_id: String) -> QueryPassages {
    self.document_id = Some(document_id);
    self
  }

  pub fn document_id(&self) -> Option<&String> {
    self.document_id.as_ref()
  }

  pub fn reset_document_id(&mut self) {
    self.document_id = None;
  }

  pub fn set_passage_score(&mut self, passage_score: f64) {
    self.passage_score = Some(passage_score);
  }

  pub fn with_passage_score(mut self, passage_score: f64) -> QueryPassages {
    self.passage_score = Some(passage_score);
    self
  }

  pub fn passage_score(&self) -> Option<&f64> {
    self.passage_score.as_ref()
  }

  pub fn reset_passage_score(&mut self) {
    self.passage_score = None;
  }

  pub fn set_passage_text(&mut self, passage_text: String) {
    self.passage_text = Some(passage_text);
  }

  pub fn with_passage_text(mut self, passage_text: String) -> QueryPassages {
    self.passage_text = Some(passage_text);
    self
  }

  pub fn passage_text(&self) -> Option<&String> {
    self.passage_text.as_ref()
  }

  pub fn reset_passage_text(&mut self) {
    self.passage_text = None;
  }

  pub fn set_start_offset(&mut self, start_offset: i32) {
    self.start_offset = Some(start_offset);
  }

  pub fn with_start_offset(mut self, start_offset: i32) -> QueryPassages {
    self.start_offset = Some(start_offset);
    self
  }

  pub fn start_offset(&self) -> Option<&i32> {
    self.start_offset.as_ref()
  }

  pub fn reset_start_offset(&mut self) {
    self.start_offset = None;
  }

  pub fn set_end_offset(&mut self, end_offset: i32) {
    self.end_offset = Some(end_offset);
  }

  pub fn with_end_offset(mut self, end_offset: i32) -> QueryPassages {
    self.end_offset = Some(end_offset);
    self
  }

  pub fn end_offset(&self) -> Option<&i32> {
    self.end_offset.as_ref()
  }

  pub fn reset_end_offset(&mut self) {
    self.end_offset = None;
  }

  pub fn set_field(&mut self, field: String) {
    self.field = Some(field);
  }

  pub fn with_field(mut self, field: String) -> QueryPassages {
    self.field = Some(field);
    self
  }

  pub fn field(&self) -> Option<&String> {
    self.field.as_ref()
  }

  pub fn reset_field(&mut self) {
    self.field = None;
  }

}



