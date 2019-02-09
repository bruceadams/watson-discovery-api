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
pub struct UpdateCollectionRequest {
  /// The name of the collection.
  #[serde(rename = "name")]
  name: String,
  /// A description of the collection.
  #[serde(rename = "description")]
  description: Option<String>,
  /// The ID of the configuration in which the collection is to be updated.
  #[serde(rename = "configuration_id")]
  configuration_id: Option<String>
}

impl UpdateCollectionRequest {
  pub fn new(name: String) -> UpdateCollectionRequest {
    UpdateCollectionRequest {
      name: name,
      description: None,
      configuration_id: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> UpdateCollectionRequest {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> UpdateCollectionRequest {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_configuration_id(&mut self, configuration_id: String) {
    self.configuration_id = Some(configuration_id);
  }

  pub fn with_configuration_id(mut self, configuration_id: String) -> UpdateCollectionRequest {
    self.configuration_id = Some(configuration_id);
    self
  }

  pub fn configuration_id(&self) -> Option<&String> {
    self.configuration_id.as_ref()
  }

  pub fn reset_configuration_id(&mut self) {
    self.configuration_id = None;
  }

}


