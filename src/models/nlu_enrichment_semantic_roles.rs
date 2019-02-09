/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NluEnrichmentSemanticRoles : An object specifiying the semantic roles enrichment and related parameters.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NluEnrichmentSemanticRoles {
  /// When `true`, entities are extracted from the identified sentence parts.
  #[serde(rename = "entities")]
  entities: Option<bool>,
  /// When `true`, keywords are extracted from the identified sentence parts.
  #[serde(rename = "keywords")]
  keywords: Option<bool>,
  /// The maximum number of semantic roles enrichments to extact from each instance of the specified field.
  #[serde(rename = "limit")]
  limit: Option<i32>
}

impl NluEnrichmentSemanticRoles {
  /// An object specifiying the semantic roles enrichment and related parameters.
  pub fn new() -> NluEnrichmentSemanticRoles {
    NluEnrichmentSemanticRoles {
      entities: None,
      keywords: None,
      limit: None
    }
  }

  pub fn set_entities(&mut self, entities: bool) {
    self.entities = Some(entities);
  }

  pub fn with_entities(mut self, entities: bool) -> NluEnrichmentSemanticRoles {
    self.entities = Some(entities);
    self
  }

  pub fn entities(&self) -> Option<&bool> {
    self.entities.as_ref()
  }

  pub fn reset_entities(&mut self) {
    self.entities = None;
  }

  pub fn set_keywords(&mut self, keywords: bool) {
    self.keywords = Some(keywords);
  }

  pub fn with_keywords(mut self, keywords: bool) -> NluEnrichmentSemanticRoles {
    self.keywords = Some(keywords);
    self
  }

  pub fn keywords(&self) -> Option<&bool> {
    self.keywords.as_ref()
  }

  pub fn reset_keywords(&mut self) {
    self.keywords = None;
  }

  pub fn set_limit(&mut self, limit: i32) {
    self.limit = Some(limit);
  }

  pub fn with_limit(mut self, limit: i32) -> NluEnrichmentSemanticRoles {
    self.limit = Some(limit);
    self
  }

  pub fn limit(&self) -> Option<&i32> {
    self.limit.as_ref()
  }

  pub fn reset_limit(&mut self) {
    self.limit = None;
  }

}



