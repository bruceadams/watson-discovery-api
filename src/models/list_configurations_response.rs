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
pub struct ListConfigurationsResponse {
  /// An array of Configurations that are available for the service instance.
  #[serde(rename = "configurations")]
  configurations: Option<Vec<::models::Configuration>>
}

impl ListConfigurationsResponse {
  pub fn new() -> ListConfigurationsResponse {
    ListConfigurationsResponse {
      configurations: None
    }
  }

  pub fn set_configurations(&mut self, configurations: Vec<::models::Configuration>) {
    self.configurations = Some(configurations);
  }

  pub fn with_configurations(mut self, configurations: Vec<::models::Configuration>) -> ListConfigurationsResponse {
    self.configurations = Some(configurations);
    self
  }

  pub fn configurations(&self) -> Option<&Vec<::models::Configuration>> {
    self.configurations.as_ref()
  }

  pub fn reset_configurations(&mut self) {
    self.configurations = None;
  }

}


