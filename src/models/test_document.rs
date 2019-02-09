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
pub struct TestDocument {
  /// The unique identifier for the configuration.
  #[serde(rename = "configuration_id")]
  configuration_id: Option<String>,
  /// Status of the preview operation.
  #[serde(rename = "status")]
  status: Option<String>,
  /// The number of 10-kB chunks of field data that were enriched. This can be used to estimate the cost of running a real ingestion.
  #[serde(rename = "enriched_field_units")]
  enriched_field_units: Option<i32>,
  /// Format of the test document.
  #[serde(rename = "original_media_type")]
  original_media_type: Option<String>,
  /// An array of objects that describe each step in the preview process.
  #[serde(rename = "snapshots")]
  snapshots: Option<Vec<::models::DocumentSnapshot>>,
  /// An array of notice messages about the preview operation.
  #[serde(rename = "notices")]
  notices: Option<Vec<::models::Notice>>
}

impl TestDocument {
  pub fn new() -> TestDocument {
    TestDocument {
      configuration_id: None,
      status: None,
      enriched_field_units: None,
      original_media_type: None,
      snapshots: None,
      notices: None
    }
  }

  pub fn set_configuration_id(&mut self, configuration_id: String) {
    self.configuration_id = Some(configuration_id);
  }

  pub fn with_configuration_id(mut self, configuration_id: String) -> TestDocument {
    self.configuration_id = Some(configuration_id);
    self
  }

  pub fn configuration_id(&self) -> Option<&String> {
    self.configuration_id.as_ref()
  }

  pub fn reset_configuration_id(&mut self) {
    self.configuration_id = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> TestDocument {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_enriched_field_units(&mut self, enriched_field_units: i32) {
    self.enriched_field_units = Some(enriched_field_units);
  }

  pub fn with_enriched_field_units(mut self, enriched_field_units: i32) -> TestDocument {
    self.enriched_field_units = Some(enriched_field_units);
    self
  }

  pub fn enriched_field_units(&self) -> Option<&i32> {
    self.enriched_field_units.as_ref()
  }

  pub fn reset_enriched_field_units(&mut self) {
    self.enriched_field_units = None;
  }

  pub fn set_original_media_type(&mut self, original_media_type: String) {
    self.original_media_type = Some(original_media_type);
  }

  pub fn with_original_media_type(mut self, original_media_type: String) -> TestDocument {
    self.original_media_type = Some(original_media_type);
    self
  }

  pub fn original_media_type(&self) -> Option<&String> {
    self.original_media_type.as_ref()
  }

  pub fn reset_original_media_type(&mut self) {
    self.original_media_type = None;
  }

  pub fn set_snapshots(&mut self, snapshots: Vec<::models::DocumentSnapshot>) {
    self.snapshots = Some(snapshots);
  }

  pub fn with_snapshots(mut self, snapshots: Vec<::models::DocumentSnapshot>) -> TestDocument {
    self.snapshots = Some(snapshots);
    self
  }

  pub fn snapshots(&self) -> Option<&Vec<::models::DocumentSnapshot>> {
    self.snapshots.as_ref()
  }

  pub fn reset_snapshots(&mut self) {
    self.snapshots = None;
  }

  pub fn set_notices(&mut self, notices: Vec<::models::Notice>) {
    self.notices = Some(notices);
  }

  pub fn with_notices(mut self, notices: Vec<::models::Notice>) -> TestDocument {
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


