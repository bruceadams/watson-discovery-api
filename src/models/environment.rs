/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Environment : Details about an environment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
  /// Unique identifier for the environment.
  #[serde(rename = "environment_id")]
  environment_id: Option<String>,
  /// Name that identifies the environment.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Description of the environment.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Creation date of the environment, in the format `yyyy-MM-dd'T'HH:mm:ss.SSS'Z'`
  #[serde(rename = "created")]
  created: Option<String>,
  /// Date of most recent environment update, in the format `yyyy-MM-dd'T'HH:mm:ss.SSS'Z'`
  #[serde(rename = "updated")]
  updated: Option<String>,
  /// Current status of the environment. `resizing` is displayed when a request to increase the environment size has been made, but is still in the process of being completed.
  #[serde(rename = "status")]
  status: Option<String>,
  /// If `true`, the environment contains read-only collections that are maintained by IBM.
  #[serde(rename = "read_only")]
  read_only: Option<bool>,
  /// Current size of the environment.
  #[serde(rename = "size")]
  size: Option<String>,
  /// The new size requested for this environment. Only returned when the environment *status* is `resizing`.  *Note:* Querying and indexing can still be performed during an environment upsize.
  #[serde(rename = "requested_size")]
  requested_size: Option<String>,
  #[serde(rename = "index_capacity")]
  index_capacity: Option<::models::IndexCapacity>,
  #[serde(rename = "search_status")]
  search_status: Option<::models::SearchStatus>
}

impl Environment {
  /// Details about an environment.
  pub fn new() -> Environment {
    Environment {
      environment_id: None,
      name: None,
      description: None,
      created: None,
      updated: None,
      status: None,
      read_only: None,
      size: None,
      requested_size: None,
      index_capacity: None,
      search_status: None
    }
  }

  pub fn set_environment_id(&mut self, environment_id: String) {
    self.environment_id = Some(environment_id);
  }

  pub fn with_environment_id(mut self, environment_id: String) -> Environment {
    self.environment_id = Some(environment_id);
    self
  }

  pub fn environment_id(&self) -> Option<&String> {
    self.environment_id.as_ref()
  }

  pub fn reset_environment_id(&mut self) {
    self.environment_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Environment {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> Environment {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> Environment {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_updated(&mut self, updated: String) {
    self.updated = Some(updated);
  }

  pub fn with_updated(mut self, updated: String) -> Environment {
    self.updated = Some(updated);
    self
  }

  pub fn updated(&self) -> Option<&String> {
    self.updated.as_ref()
  }

  pub fn reset_updated(&mut self) {
    self.updated = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> Environment {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> Environment {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_size(&mut self, size: String) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: String) -> Environment {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&String> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_requested_size(&mut self, requested_size: String) {
    self.requested_size = Some(requested_size);
  }

  pub fn with_requested_size(mut self, requested_size: String) -> Environment {
    self.requested_size = Some(requested_size);
    self
  }

  pub fn requested_size(&self) -> Option<&String> {
    self.requested_size.as_ref()
  }

  pub fn reset_requested_size(&mut self) {
    self.requested_size = None;
  }

  pub fn set_index_capacity(&mut self, index_capacity: ::models::IndexCapacity) {
    self.index_capacity = Some(index_capacity);
  }

  pub fn with_index_capacity(mut self, index_capacity: ::models::IndexCapacity) -> Environment {
    self.index_capacity = Some(index_capacity);
    self
  }

  pub fn index_capacity(&self) -> Option<&::models::IndexCapacity> {
    self.index_capacity.as_ref()
  }

  pub fn reset_index_capacity(&mut self) {
    self.index_capacity = None;
  }

  pub fn set_search_status(&mut self, search_status: ::models::SearchStatus) {
    self.search_status = Some(search_status);
  }

  pub fn with_search_status(mut self, search_status: ::models::SearchStatus) -> Environment {
    self.search_status = Some(search_status);
    self
  }

  pub fn search_status(&self) -> Option<&::models::SearchStatus> {
    self.search_status.as_ref()
  }

  pub fn reset_search_status(&mut self) {
    self.search_status = None;
  }

}


