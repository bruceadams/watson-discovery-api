/* 
 * Discovery
 *
 * The IBM Watson&trade; Discovery Service is a cognitive search and content analytics engine that you can add to applications to identify patterns, trends and actionable insights to drive better decision-making. Securely unify structured and unstructured data with pre-enriched content, and use a simplified query language to eliminate the need for manual filtering of results.
 *
 * OpenAPI spec version: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnrichmentOptions : Options which are specific to a particular enrichment.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichmentOptions {
  #[serde(rename = "features")]
  features: Option<::models::NluEnrichmentFeatures>,
  /// ISO 639-1 code indicating the language to use for the analysis. This code overrides the automatic language detection performed by the service. Valid codes are `ar` (Arabic), `en` (English), `fr` (French), `de` (German), `it` (Italian), `pt` (Portuguese), `ru` (Russian), `es` (Spanish), and `sv` (Swedish). **Note:** Not all features support all languages, automatic detection is recommended.
  #[serde(rename = "language")]
  language: Option<String>,
  /// *For use with `elements` enrichments only.* The element extraction model to use. Models available are: `contract`.
  #[serde(rename = "model")]
  model: Option<String>
}

impl EnrichmentOptions {
  /// Options which are specific to a particular enrichment.
  pub fn new() -> EnrichmentOptions {
    EnrichmentOptions {
      features: None,
      language: None,
      model: None
    }
  }

  pub fn set_features(&mut self, features: ::models::NluEnrichmentFeatures) {
    self.features = Some(features);
  }

  pub fn with_features(mut self, features: ::models::NluEnrichmentFeatures) -> EnrichmentOptions {
    self.features = Some(features);
    self
  }

  pub fn features(&self) -> Option<&::models::NluEnrichmentFeatures> {
    self.features.as_ref()
  }

  pub fn reset_features(&mut self) {
    self.features = None;
  }

  pub fn set_language(&mut self, language: String) {
    self.language = Some(language);
  }

  pub fn with_language(mut self, language: String) -> EnrichmentOptions {
    self.language = Some(language);
    self
  }

  pub fn language(&self) -> Option<&String> {
    self.language.as_ref()
  }

  pub fn reset_language(&mut self) {
    self.language = None;
  }

  pub fn set_model(&mut self, model: String) {
    self.model = Some(model);
  }

  pub fn with_model(mut self, model: String) -> EnrichmentOptions {
    self.model = Some(model);
    self
  }

  pub fn model(&self) -> Option<&String> {
    self.model.as_ref()
  }

  pub fn reset_model(&mut self) {
    self.model = None;
  }

}



