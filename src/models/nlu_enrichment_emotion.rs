/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NluEnrichmentEmotion : An object specifying the emotion detection enrichment and related parameters.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NluEnrichmentEmotion {
  /// When `true`, emotion detection is performed on the entire field.
  #[serde(rename = "document")]
  document: Option<bool>,
  /// A comma-separated list of target strings that will have any associated emotions detected.
  #[serde(rename = "targets")]
  targets: Option<Vec<String>>
}

impl NluEnrichmentEmotion {
  /// An object specifying the emotion detection enrichment and related parameters.
  pub fn new() -> NluEnrichmentEmotion {
    NluEnrichmentEmotion {
      document: None,
      targets: None
    }
  }

  pub fn set_document(&mut self, document: bool) {
    self.document = Some(document);
  }

  pub fn with_document(mut self, document: bool) -> NluEnrichmentEmotion {
    self.document = Some(document);
    self
  }

  pub fn document(&self) -> Option<&bool> {
    self.document.as_ref()
  }

  pub fn reset_document(&mut self) {
    self.document = None;
  }

  pub fn set_targets(&mut self, targets: Vec<String>) {
    self.targets = Some(targets);
  }

  pub fn with_targets(mut self, targets: Vec<String>) -> NluEnrichmentEmotion {
    self.targets = Some(targets);
    self
  }

  pub fn targets(&self) -> Option<&Vec<String>> {
    self.targets.as_ref()
  }

  pub fn reset_targets(&mut self) {
    self.targets = None;
  }

}



