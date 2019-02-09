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
pub struct DeleteDocumentResponse {
  /// The unique identifier of the document.
  #[serde(rename = "document_id")]
  document_id: Option<String>,
  /// Status of the document. A deleted document has the status deleted.
  #[serde(rename = "status")]
  status: Option<String>
}

impl DeleteDocumentResponse {
  pub fn new() -> DeleteDocumentResponse {
    DeleteDocumentResponse {
      document_id: None,
      status: None
    }
  }

  pub fn set_document_id(&mut self, document_id: String) {
    self.document_id = Some(document_id);
  }

  pub fn with_document_id(mut self, document_id: String) -> DeleteDocumentResponse {
    self.document_id = Some(document_id);
    self
  }

  pub fn document_id(&self) -> Option<&String> {
    self.document_id.as_ref()
  }

  pub fn reset_document_id(&mut self) {
    self.document_id = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> DeleteDocumentResponse {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}


