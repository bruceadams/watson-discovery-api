/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PdfSettings : A list of PDF conversion settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfSettings {
  #[serde(rename = "heading")]
  heading: Option<::models::PdfHeadingDetection>
}

impl PdfSettings {
  /// A list of PDF conversion settings.
  pub fn new() -> PdfSettings {
    PdfSettings {
      heading: None
    }
  }

  pub fn set_heading(&mut self, heading: ::models::PdfHeadingDetection) {
    self.heading = Some(heading);
  }

  pub fn with_heading(mut self, heading: ::models::PdfHeadingDetection) -> PdfSettings {
    self.heading = Some(heading);
    self
  }

  pub fn heading(&self) -> Option<&::models::PdfHeadingDetection> {
    self.heading.as_ref()
  }

  pub fn reset_heading(&mut self) {
    self.heading = None;
  }

}



