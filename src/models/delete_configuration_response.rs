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
pub struct DeleteConfigurationResponse {
  /// The unique identifier for the configuration.
  #[serde(rename = "configuration_id")]
  configuration_id: String,
  /// Status of the configuration. A deleted configuration has the status deleted.
  #[serde(rename = "status")]
  status: String,
  /// An array of notice messages, if any.
  #[serde(rename = "notices")]
  notices: Option<Vec<::models::Notice>>
}

impl DeleteConfigurationResponse {
  pub fn new(configuration_id: String, status: String) -> DeleteConfigurationResponse {
    DeleteConfigurationResponse {
      configuration_id: configuration_id,
      status: status,
      notices: None
    }
  }

  pub fn set_configuration_id(&mut self, configuration_id: String) {
    self.configuration_id = configuration_id;
  }

  pub fn with_configuration_id(mut self, configuration_id: String) -> DeleteConfigurationResponse {
    self.configuration_id = configuration_id;
    self
  }

  pub fn configuration_id(&self) -> &String {
    &self.configuration_id
  }


  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> DeleteConfigurationResponse {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_notices(&mut self, notices: Vec<::models::Notice>) {
    self.notices = Some(notices);
  }

  pub fn with_notices(mut self, notices: Vec<::models::Notice>) -> DeleteConfigurationResponse {
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


