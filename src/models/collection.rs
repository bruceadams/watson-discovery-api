/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Collection : A collection for storing documents.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
  /// The unique identifier of the collection.
  #[serde(rename = "collection_id")]
  collection_id: Option<String>,
  /// The name of the collection.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The description of the collection.
  #[serde(rename = "description")]
  description: Option<String>,
  /// The creation date of the collection in the format yyyy-MM-dd'T'HH:mmcon:ss.SSS'Z'
  #[serde(rename = "created")]
  created: Option<String>,
  /// The timestamp of when the collection was last updated in the format yyyy-MM-dd'T'HH:mm:ss.SSS'Z'
  #[serde(rename = "updated")]
  updated: Option<String>,
  /// The status of the collection.
  #[serde(rename = "status")]
  status: Option<String>,
  /// The unique identifier of the collection's configuration.
  #[serde(rename = "configuration_id")]
  configuration_id: Option<String>,
  /// The language of the documents stored in the collection. Permitted values include `en` (English), `de` (German), and `es` (Spanish).
  #[serde(rename = "language")]
  language: Option<String>,
  #[serde(rename = "document_counts")]
  document_counts: Option<::models::DocumentCounts>,
  #[serde(rename = "disk_usage")]
  disk_usage: Option<::models::CollectionDiskUsage>,
  #[serde(rename = "training_status")]
  training_status: Option<::models::TrainingStatus>,
  #[serde(rename = "source_crawl")]
  source_crawl: Option<::models::SourceStatus>
}

impl Collection {
  /// A collection for storing documents.
  pub fn new() -> Collection {
    Collection {
      collection_id: None,
      name: None,
      description: None,
      created: None,
      updated: None,
      status: None,
      configuration_id: None,
      language: None,
      document_counts: None,
      disk_usage: None,
      training_status: None,
      source_crawl: None
    }
  }

  pub fn set_collection_id(&mut self, collection_id: String) {
    self.collection_id = Some(collection_id);
  }

  pub fn with_collection_id(mut self, collection_id: String) -> Collection {
    self.collection_id = Some(collection_id);
    self
  }

  pub fn collection_id(&self) -> Option<&String> {
    self.collection_id.as_ref()
  }

  pub fn reset_collection_id(&mut self) {
    self.collection_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> Collection {
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

  pub fn with_description(mut self, description: String) -> Collection {
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

  pub fn with_created(mut self, created: String) -> Collection {
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

  pub fn with_updated(mut self, updated: String) -> Collection {
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

  pub fn with_status(mut self, status: String) -> Collection {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_configuration_id(&mut self, configuration_id: String) {
    self.configuration_id = Some(configuration_id);
  }

  pub fn with_configuration_id(mut self, configuration_id: String) -> Collection {
    self.configuration_id = Some(configuration_id);
    self
  }

  pub fn configuration_id(&self) -> Option<&String> {
    self.configuration_id.as_ref()
  }

  pub fn reset_configuration_id(&mut self) {
    self.configuration_id = None;
  }

  pub fn set_language(&mut self, language: String) {
    self.language = Some(language);
  }

  pub fn with_language(mut self, language: String) -> Collection {
    self.language = Some(language);
    self
  }

  pub fn language(&self) -> Option<&String> {
    self.language.as_ref()
  }

  pub fn reset_language(&mut self) {
    self.language = None;
  }

  pub fn set_document_counts(&mut self, document_counts: ::models::DocumentCounts) {
    self.document_counts = Some(document_counts);
  }

  pub fn with_document_counts(mut self, document_counts: ::models::DocumentCounts) -> Collection {
    self.document_counts = Some(document_counts);
    self
  }

  pub fn document_counts(&self) -> Option<&::models::DocumentCounts> {
    self.document_counts.as_ref()
  }

  pub fn reset_document_counts(&mut self) {
    self.document_counts = None;
  }

  pub fn set_disk_usage(&mut self, disk_usage: ::models::CollectionDiskUsage) {
    self.disk_usage = Some(disk_usage);
  }

  pub fn with_disk_usage(mut self, disk_usage: ::models::CollectionDiskUsage) -> Collection {
    self.disk_usage = Some(disk_usage);
    self
  }

  pub fn disk_usage(&self) -> Option<&::models::CollectionDiskUsage> {
    self.disk_usage.as_ref()
  }

  pub fn reset_disk_usage(&mut self) {
    self.disk_usage = None;
  }

  pub fn set_training_status(&mut self, training_status: ::models::TrainingStatus) {
    self.training_status = Some(training_status);
  }

  pub fn with_training_status(mut self, training_status: ::models::TrainingStatus) -> Collection {
    self.training_status = Some(training_status);
    self
  }

  pub fn training_status(&self) -> Option<&::models::TrainingStatus> {
    self.training_status.as_ref()
  }

  pub fn reset_training_status(&mut self) {
    self.training_status = None;
  }

  pub fn set_source_crawl(&mut self, source_crawl: ::models::SourceStatus) {
    self.source_crawl = Some(source_crawl);
  }

  pub fn with_source_crawl(mut self, source_crawl: ::models::SourceStatus) -> Collection {
    self.source_crawl = Some(source_crawl);
    self
  }

  pub fn source_crawl(&self) -> Option<&::models::SourceStatus> {
    self.source_crawl.as_ref()
  }

  pub fn reset_source_crawl(&mut self) {
    self.source_crawl = None;
  }

}



